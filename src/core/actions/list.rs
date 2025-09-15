use std::{
    collections::{BinaryHeap, VecDeque},
    error::Error,
    fs::File,
    io::{BufRead, BufReader, IsTerminal, Write},
    path::{Path, PathBuf},
};

use chrono::{DateTime, Local};
use termcolor::{Color, StandardStream, WriteColor};

use crate::{
    core::markdown::NotesFrontMatter,
    system::{self, Configuration},
    write_coloured, write_colouredln,
};

use super::{Command, Commands};

#[derive(Debug)]
enum Details {
    Root,
    Short,
    Full,
    Default,
}

struct ListEntry {
    path: PathBuf,
    frontmatter: NotesFrontMatter,
    contents: BufReader<File>,
}

impl Eq for ListEntry {}

impl Ord for ListEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.frontmatter.date.cmp(&other.frontmatter.date).then(
            self.frontmatter
                .notes_metadata
                .category
                .cmp(&other.frontmatter.notes_metadata.category),
        )
    }
}

impl PartialOrd for ListEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ListEntry {
    fn eq(&self, other: &Self) -> bool {
        self.frontmatter.date == other.frontmatter.date
            && self.frontmatter.notes_metadata.category == other.frontmatter.notes_metadata.category
    }
}

pub struct ListCommand {
    path: PathBuf,
    details: Details,
    entries: BinaryHeap<ListEntry>,
}

impl Command<'_> for ListCommand {
    fn new(args: Commands, conf: &Configuration) -> Result<Self, Box<dyn Error>> {
        let Commands::List {
            root,
            full,
            short,
            category,
        } = args
        else {
            unreachable!("Non-list command passed to list handler.");
        };

        if root {
            return Ok(Self {
                path: PathBuf::from(&conf.settings.path),
                details: Details::Root,
                entries: BinaryHeap::<ListEntry>::new(),
            });
        }

        if let Some(options) = &conf.options {
            if options.hide_root.as_ref().is_some_and(|s| s == "true") {
                // safety: notes is a single threaded program
                unsafe { std::env::set_var("NOTES_HIDE_ROOT", "true") };
            }
        }

        // flags are represented as booleans and default to false
        let details = if short {
            Details::Short
        } else if full {
            Details::Full
        } else {
            Details::Default
        };
        Ok(if let Some(cat) = category {
            Self {
                details,
                path: PathBuf::from(format!("{}/{}", conf.settings.path, cat)),
                entries: BinaryHeap::<ListEntry>::new(),
            }
        } else {
            Self {
                details,
                path: PathBuf::from(&conf.settings.path),
                entries: BinaryHeap::<ListEntry>::new(),
            }
        })
    }

    fn execute(mut self) -> Result<(), Box<dyn Error>> {
        if let Details::Root = &self.details {
            return Ok(writeln!(
                std::io::stdout(),
                "{}",
                self.path
                    .as_os_str()
                    .to_str()
                    .expect("only valid UTF-8 characters are used for the path in configuration")
            )?);
        }

        let (namelen, taglen) = root_bfs_walk(self.path, &mut self.entries)?;
        match self.details {
            Details::Short => {
                short_handler(self.entries, namelen, taglen)?;
            }
            Details::Full => full_handler(self.entries)?,
            Details::Default => default_handler(self.entries)?,
            _ => unreachable!(),
        };

        Ok(())
    }
}

/// Utility function for fetching the yaml front matter of a note
fn fetch_front_matter(reader: &mut BufReader<File>) -> Result<String, Box<dyn Error>> {
    let lines = reader.lines();
    let mut front_matter = String::new();
    let mut in_front_matter = false;
    for line in lines {
        let string = line?;
        if string.as_str().trim() == "---" {
            if in_front_matter {
                break;
            }
            in_front_matter = true;
            continue;
        };
        front_matter.push_str(&string);
        front_matter.push('\n');
    }
    Ok(front_matter)
}

/// Deserialises a str slice into frontmatter
fn generate_frontmatter(buf: &str) -> Result<NotesFrontMatter, Box<dyn Error>> {
    Ok(serde_yaml_ng::from_str::<NotesFrontMatter>(buf)?)
}

/// Root directory traversal that populates the list entries queue and returns padding for tags
/// and pathnames.
fn root_bfs_walk(
    root_path: PathBuf,
    entries: &mut BinaryHeap<ListEntry>,
) -> Result<(usize, usize), Box<dyn Error>> {
    let mut dequeue = VecDeque::new();
    let mut lengths: (usize, usize) = (0, 0);
    dequeue.push_back(root_path);
    while !dequeue.is_empty() {
        let entry = dequeue
            .pop_front()
            .expect("invariant: rootpath not provided to root_bfs_walk");
        for child in std::fs::read_dir(entry)? {
            let child = child?;
            let path = child.path();
            if path.is_dir() {
                dequeue.push_back(path);
            } else {
                let path_str = path
                    .to_str()
                    .expect("Invalid UTF-8 sequence provided as path");
                if path_str.is_ascii() && &path_str[path_str.len() - 2..] == "md"
                    || path_str.chars().rev().take(2).collect::<String>() == "md"
                {
                    let mut reader = BufReader::new(std::fs::File::open(&path)?);
                    let frontmatter = fetch_front_matter(&mut reader)?;
                    let new_entry = ListEntry {
                        path,
                        frontmatter: generate_frontmatter(&frontmatter)?,
                        contents: reader,
                    };
                    let (namelen, taglen) = &mut lengths;
                    (*namelen, *taglen) = compute_name_and_tag_widths(
                        &new_entry.path,
                        &new_entry.frontmatter,
                        *namelen,
                        *taglen,
                    )?;
                    if !new_entry.frontmatter.notes_metadata.hidden {
                        entries.push(new_entry);
                    }
                }
            }
        }
    }
    Ok(lengths)
}

/// Computes padding size for tags and name
fn compute_name_and_tag_widths(
    path: &Path,
    frontmatter: &NotesFrontMatter,
    mut namelen: usize,
    mut taglen: usize,
) -> Result<(usize, usize), Box<dyn Error>> {
    if frontmatter.notes_metadata.hidden {
        return Ok((namelen, taglen));
    }

    const PAD: usize = 2;
    let ncount = path
        .iter()
        .skip_while(|&p| p != "notes")
        .skip(1)
        .fold(String::new(), |mut s, e| {
            s += e.to_str().unwrap();
            s += "/";
            s
        })
        .chars()
        .count();
    let mut iter = path.iter();
    iter.next_back();
    let tcount = if let Some(tags) = &frontmatter.tags {
        tags.iter()
            .fold(0, |sum, tag| tag.chars().count() + PAD + sum)
            - 1
    } else {
        0
    };
    if namelen < ncount {
        namelen = ncount;
    }
    if taglen < tcount {
        taglen = tcount;
    }
    Ok((namelen, taglen))
}

fn default_handler(mut entries: BinaryHeap<ListEntry>) -> Result<(), Box<dyn Error>> {
    while let Some(entry) = entries.pop() {
        let NotesFrontMatter {
            title: _,
            date: _,
            tags: _,
            notes_metadata,
        } = &entry.frontmatter;

        if notes_metadata.hidden {
            continue;
        }

        if std::io::stdout().is_terminal()
            && std::env::var("NOTES_HIDE_ROOT").is_ok_and(|s| s == "true")
        {
            let mut shortened_path = String::new();
            let root_path_length = system::DATA_DIR.chars().count();
            for (i, c) in system::DATA_DIR.chars().enumerate() {
                if c == '/' && i != root_path_length - 1 {
                    shortened_path.push(c);
                    shortened_path.push(system::DATA_DIR.chars().nth(i + 1).unwrap());
                }
            }
            writeln!(
                std::io::stdout(),
                "{shortened_path}{}",
                entry
                    .path
                    .to_str()
                    .unwrap()
                    .split(system::DATA_DIR)
                    .last()
                    .unwrap()
            )?;
        } else {
            let path_str = entry
                .path
                .to_str()
                .expect("An invalid UTF-8 sequence provided as a path");
            writeln!(std::io::stdout(), "{}", path_str)?;
        }
    }
    Ok(())
}

fn full_handler(mut entries: BinaryHeap<ListEntry>) -> Result<(), Box<dyn Error>> {
    let mut out = StandardStream::stdout(termcolor::ColorChoice::Always);
    while let Some(entry) = entries.pop() {
        let NotesFrontMatter {
            title: _,
            date,
            tags,
            notes_metadata,
        } = &entry.frontmatter;

        if notes_metadata.hidden {
            continue;
        }

        if std::io::stdout().is_terminal()
            && std::env::var("NOTES_HIDE_ROOT").is_ok_and(|s| s == "true")
        {
            let mut shortened_path = String::new();
            let root_path_length = system::DATA_DIR.chars().count();
            for (i, c) in system::DATA_DIR.chars().enumerate() {
                if c == '/' && i != root_path_length - 1 {
                    shortened_path.push(c);
                    shortened_path.push(system::DATA_DIR.chars().nth(i + 1).unwrap());
                }
            }
            write_colouredln!(
                out,
                colour = Color::Green,
                "{shortened_path}{}",
                entry
                    .path
                    .to_str()
                    .unwrap()
                    .split(system::DATA_DIR)
                    .last()
                    .unwrap()
            );
        } else {
            write_colouredln!(
                out,
                colour = Color::Green,
                "{}",
                entry.path.to_str().unwrap()
            );
        }

        const CATPAD: usize = 1;
        const TAGPAD: usize = 5;
        const DATEPAD: usize = 5;

        write_coloured!(out, bold_colour = Color::Yellow, "category:",);
        if let Some(category) = &notes_metadata.category {
            writeln!(
                out,
                "{:>gap$}",
                category,
                gap = category.chars().count() + CATPAD
            )?;
        } else {
            writeln!(out)?;
        }

        write_coloured!(out, bold_colour = Color::Yellow, "tags:");
        if let Some(tags) = &tags {
            let tags = tags.join(",");
            let count = tags.chars().count();
            writeln!(out, "{:>gap$}", tags, gap = count + TAGPAD)?;
        } else {
            writeln!(out)?;
        }

        write_coloured!(out, bold_colour = Color::Yellow, "date:",);
        let dt: DateTime<Local> = date.parse()?;
        let formatted_dt = dt.format("%d-%b-%Y %H:%M:%S %P %z").to_string();
        writeln!(
            out,
            "{:>gap$}",
            formatted_dt,
            gap = formatted_dt.chars().count() + DATEPAD
        )?;

        let lines = entry.contents.lines();
        for l in lines {
            writeln!(out, "{}", l?)?;
        }

        writeln!(out)?;
    }
    Ok(())
}

fn short_handler(
    mut entries: BinaryHeap<ListEntry>,
    nlen: usize,
    taglen: usize,
) -> Result<(), Box<dyn Error>> {
    let mut out = StandardStream::stdout(termcolor::ColorChoice::Always);
    while let Some(entry) = entries.pop() {
        let NotesFrontMatter {
            title: _,
            date,
            tags,
            notes_metadata,
        } = &entry.frontmatter;

        if notes_metadata.hidden {
            continue;
        }

        let pbuf = entry.path;
        let mut iter = pbuf.iter();
        let file = iter.next_back();

        let mut gap = nlen;
        if let Some(category) = &notes_metadata.category {
            write_coloured!(out, colour = Color::Green, "/{}", category);
            gap = nlen - category.chars().count();
            write_coloured!(out, bold, "/");
        } else {
            write_coloured!(out, colour = Color::Green, "/");
            gap += 1
        };

        write_coloured!(
            out,
            colour = Color::Yellow,
            "{:<gap$}",
            file.unwrap().to_str().unwrap(),
        );

        let gap = taglen + 2;
        if let Some(tags) = &tags {
            write_coloured!(out, bold, " {:<gap$}", tags.join(","));
        } else {
            write!(out, " {:<gap$}", "")?;
        }
        let dt: DateTime<Local> = date.parse()?;
        write_colouredln!(
            out,
            bold,
            "{}",
            dt.format("%d-%b-%Y %H:%M:%S %P %z").to_string()
        );
    }
    Ok(())
}
