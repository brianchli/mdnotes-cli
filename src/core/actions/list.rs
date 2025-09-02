use std::{
    error::Error,
    fs::{DirEntry, File},
    io::{BufRead, BufReader, IsTerminal, Write},
    path::{Path, PathBuf},
};

use clap::ArgMatches;
use termcolor::{Color, ColorSpec, StandardStream, WriteColor};

use crate::{
    core::markdown::{Metadata, NotesFrontMatter},
    system::{self, Configuration},
    write_coloured, write_colouredln,
};

use super::Command;

#[derive(Debug)]
enum Details {
    Root,
    Short,
    Full,
    Default,
}

pub struct ListCommand {
    path: PathBuf,
    details: Details,
}

impl Command<'_> for ListCommand {
    fn new(args: &ArgMatches, conf: &Configuration) -> Result<Self, Box<dyn Error>> {
        if args.get_one::<bool>("root").is_some_and(|&b| b) {
            return Ok(Self {
                path: PathBuf::from(&conf.settings.path),
                details: Details::Root,
            });
        }

        if let Some(options) = &conf.options {
            if options.hide_root.as_ref().is_some_and(|s| s == "true") {
                // safety: notes is a single threaded program
                unsafe { std::env::set_var("NOTES_HIDE_ROOT", "true") };
            }
        }

        // flags are represented as booleans and default to false
        let details = if args.get_one("short").is_some_and(|&v| v) {
            Details::Short
        } else if args.get_one::<bool>("full").is_some_and(|&v| v) {
            Details::Full
        } else {
            Details::Default
        };
        Ok(if let Some(cat) = args.get_one::<String>("category") {
            Self {
                details,
                path: PathBuf::from(format!("{}/{}", conf.settings.path, cat)),
            }
        } else {
            Self {
                details,
                path: PathBuf::from(&conf.settings.path),
            }
        })
    }

    fn execute(&self) -> Result<(), Box<dyn Error>> {
        match self.details {
            Details::Root => Ok(writeln!(
                std::io::stdout(),
                "{}",
                self.path
                    .as_os_str()
                    .to_str()
                    .expect("only valid UTF-8 characters are used for the path in configuration")
            )?),
            Details::Default => Ok(walk_dir(&self.path, &default_cb)?),
            Details::Full => Ok(walk_dir(&self.path, &full_cb)?),
            Details::Short => {
                let (namelen, taglen) = max_name_and_tag_len(None, &self.path)?;
                let cb = |dir: &DirEntry| -> Result<(), Box<dyn Error>> {
                    short_cb(dir, namelen, taglen)
                };
                Ok(walk_dir(&self.path, &cb)?)
            }
        }
    }
}

/// Walks the directory and applies the callback function.
/// code snippet taken from std::fs::read_dir documentation.
fn walk_dir(
    dir: &Path,
    cb: &impl Fn(&DirEntry) -> Result<(), Box<dyn Error>>,
) -> Result<(), Box<dyn Error>> {
    if dir.is_dir() {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                walk_dir(&path, cb)?;
            } else {
                let path_str = path
                    .to_str()
                    .expect("Invalid UTF-8 sequence provided as path");
                if path_str.is_ascii() && &path_str[path_str.len() - 2..] == "md"
                    || path_str.chars().rev().take(2).collect::<String>() == "md"
                {
                    cb(&entry)?;
                }
            }
        }
    }
    Ok(())
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

fn compute_metadata<'a>(buf: &'a str) -> Result<Metadata<'a>, Box<dyn Error>> {
    Ok(serde_yaml_ng::from_str::<NotesFrontMatter>(buf)?.metadata)
}

/// Computes counts for padding tags and name
fn compute_counts(
    path: &Path,
    mut namelen: usize,
    mut taglen: usize,
) -> Result<(usize, usize), Box<dyn Error>> {
    let mut reader = BufReader::new(std::fs::File::open(path)?);
    let front_matter = fetch_front_matter(&mut reader)?;
    let Metadata {
        category: _,
        tags,
        created: _,
        hidden,
    } = compute_metadata(&front_matter)?;

    if hidden {
        return Ok((0, 0));
    }

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
    let tcount = if let Some(tags) = &tags {
        tags.iter()
            .fold(0, |sum, &tag| tag.chars().count() + 2 + sum)
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

/// Computes the maximum length of the name and tag strings
fn max_name_and_tag_len(
    mut max_len: Option<(usize, usize)>,
    dir: &Path,
) -> Result<(usize, usize), Box<dyn Error>> {
    if dir.is_file() {
        let (n, c) = max_len.get_or_insert_default();
        (*n, *c) = compute_counts(dir, *n, *c)?;
        return Ok(*max_len.get_or_insert_default());
    }

    assert!(dir.is_dir());

    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let (name, cat) = max_name_and_tag_len(None, path.as_path()).unwrap();
            let (n, c) = max_len.get_or_insert_default();
            if *n < name {
                *n = name;
            }
            if *c < cat {
                *c = cat;
            }
        } else {
            let path_str = path
                .to_str()
                .expect("An invalid UTF-8 sequence provided as a path");
            let (n, c) = max_len.get_or_insert_default();
            if path_str.is_ascii() && &path_str[path_str.len() - 2..] == "md"
                || &path_str.chars().rev().take(2).collect::<String>() == "md"
            {
                (*n, *c) = compute_counts(&path, *n, *c)?;
            }
        }
    }

    Ok(*max_len.get_or_insert_default())
}

fn default_cb(dir: &DirEntry) -> Result<(), Box<dyn Error>> {
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
            dir.path()
                .to_str()
                .unwrap()
                .split(system::DATA_DIR)
                .last()
                .unwrap()
        )?;
    } else {
        let path = dir.path();
        let mut reader = BufReader::new(std::fs::File::open(&path)?);
        let front_matter = fetch_front_matter(&mut reader)?;
        let Metadata {
            category: _,
            tags: _,
            created: _,
            hidden,
        } = compute_metadata(&front_matter)?.metadata;

        if hidden {
            return Ok(());
        };

        let path_str = path
            .to_str()
            .expect("An invalid UTF-8 sequence provided as a path");
        writeln!(std::io::stdout(), "{}", path_str)?;
    }
    Ok(())
}

fn full_cb(dir: &DirEntry) -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(std::fs::File::open(dir.path())?);
    let mut out = StandardStream::stdout(termcolor::ColorChoice::Always);
    let path = dir.path();
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
            path.to_str()
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
            dir.path().to_str().unwrap()
        );
    }
    let front_matter = fetch_front_matter(&mut reader)?;
    let Metadata {
        category,
        tags,
        created,
        hidden,
    } = compute_metadata(&front_matter)?.metadata;

    if hidden {
        return Ok(());
    }

    write_coloured!(out, bold_colour = Color::Yellow, "category:",);
    if let Some(category) = &category {
        writeln!(
            out,
            "{:>gap$}",
            category,
            gap = category.chars().count() + 1
        )?;
    } else {
        writeln!(out)?;
    }

    write_coloured!(out, bold_colour = Color::Yellow, "tags:");
    if let Some(tags) = &tags {
        let tags = tags.join(",");
        let count = tags.chars().count();
        writeln!(out, "{:>gap$}", tags, gap = count + 5)?;
    } else {
        writeln!(out)?;
    }

    write_coloured!(out, bold_colour = Color::Yellow, "created:",);
    writeln!(out, "{:>gap$}", created, gap = created.chars().count() + 2)?;

    let lines = reader.lines();
    for l in lines {
        writeln!(out, "{}", l?)?;
    }

    writeln!(out)?;
    Ok(())
}

fn short_cb(dir: &DirEntry, nlen: usize, taglen: usize) -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(std::fs::File::open(dir.path())?);
    let front_matter = fetch_front_matter(&mut reader)?;
    let Metadata {
        category,
        tags,
        created,
        hidden,
    } = compute_metadata(&front_matter)?.metadata;

    if hidden {
        return Ok(());
    }

    let mut out = StandardStream::stdout(termcolor::ColorChoice::Always);

    let pbuf = dir.path();
    let mut iter = pbuf.iter();
    let file = iter.next_back();

    let mut gap = nlen;
    if let Some(category) = category {
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
    if let Some(tags) = tags {
        write_coloured!(out, bold, " {:<gap$}", tags.join(","));
    } else {
        write!(out, " {:<gap$}", "")?;
    }
    write_colouredln!(out, bold, "{}", created);
    Ok(())
}
