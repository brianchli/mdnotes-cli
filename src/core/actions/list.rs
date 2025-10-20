mod handlers;

use std::{
    collections::{BinaryHeap, HashSet, VecDeque},
    error::Error,
    fs::File,
    io::BufReader,
    io::Write,
    path::{Path, PathBuf},
};

use crate::{
    core::{frontmatter, markdown::NotesFrontMatter},
    system::Configuration,
};

use super::{Command, Commands};

#[derive(Clone, Debug, clap::ValueEnum)]
pub enum Opts {
    Root,
    Short,
    Full,
    Categories,
}

struct ListEntry {
    path: PathBuf,
    frontmatter: NotesFrontMatter,
    contents: BufReader<File>,
}

pub struct ListCommand {
    path: PathBuf,
    filter: Option<String>,
    details: Option<Opts>,
    entries: BinaryHeap<ListEntry>,
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

impl Command<'_> for ListCommand {
    fn new(args: Commands, conf: &Configuration) -> Result<Self, Box<dyn Error>> {
        let Commands::List {
            root,
            full,
            short,
            category,
            categories,
        } = args
        else {
            unreachable!("Non-list command passed to list handler.");
        };

        if root {
            return Ok(Self {
                path: PathBuf::from(&conf.settings.path),
                filter: category,
                details: Some(Opts::Root),
                entries: BinaryHeap::<ListEntry>::new(),
            });
        } else if categories {
            return Ok(Self {
                path: PathBuf::from(&conf.settings.path),
                filter: category,
                details: Some(Opts::Categories),
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
            Some(Opts::Short)
        } else if full {
            Some(Opts::Full)
        } else {
            None
        };

        Ok(Self {
            details,
            filter: category,
            path: PathBuf::from(&conf.settings.path),
            entries: BinaryHeap::<ListEntry>::new(),
        })
    }

    fn execute(mut self) -> Result<(), Box<dyn Error>> {
        let (namelen, taglen) = root_bfs_walk(&mut self)?;
        match self.details {
            Some(Opts::Short) => {
                handlers::short(self.entries, namelen, taglen)?;
            }
            Some(Opts::Full) => handlers::full(self.entries)?,
            Some(Opts::Root) => {
                return Ok(writeln!(
                    std::io::stdout(),
                    "{}",
                    self.path.as_os_str().to_str().expect(
                        "only valid UTF-8 characters are used for the path in configuration"
                    )
                )?);
            }
            Some(Opts::Categories) => {
                let set = HashSet::<&str>::from_iter(
                    self.entries
                        .iter()
                        .filter(|&p| !p.frontmatter.notes_metadata.hidden)
                        .filter_map(|p| {
                            p.frontmatter
                                .notes_metadata
                                .category
                                .as_ref()
                                .map(|s| s.as_ref())
                        }),
                );
                for cat in set {
                    writeln!(std::io::stdout(), "{}", cat)?;
                }
                return Ok(());
            }
            None => handlers::default(self.entries)?,
        };

        Ok(())
    }
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

/// Root directory traversal that populates the list entries queue and returns padding for tags
/// and pathnames.
fn root_bfs_walk(list: &mut ListCommand) -> Result<(usize, usize), Box<dyn Error>> {
    let mut dequeue = VecDeque::new();
    let mut lengths: (usize, usize) = (0, 0);
    dequeue.push_back(list.path.clone());
    while let Some(entry) = dequeue.pop_front() {
        for child in std::fs::read_dir(entry)? {
            let child = child?;
            let mut path = child.path();
            if path.is_dir() {
                dequeue.push_back(path);
            } else {
                let path_str = path
                    .to_str()
                    .expect("Invalid UTF-8 sequence provided as path");
                if path_str.is_ascii() && &path_str[path_str.len() - 2..] == "md"
                    || path_str.chars().rev().take(2).collect::<String>() == "md"
                {
                    if let Some(cat) = &list.filter {
                        path.set_extension("");
                        if !path.to_str().unwrap().contains(cat) {
                            continue;
                        }
                        path.set_extension("md");
                    }
                    let mut reader = BufReader::new(std::fs::File::open(&path)?);
                    let frontmatter = frontmatter::fetch(&mut reader)?;
                    let new_entry = ListEntry {
                        path,
                        frontmatter: frontmatter::generate(&frontmatter)?,
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
                        list.entries.push(new_entry);
                    }
                }
            }
        }
    }
    Ok(lengths)
}
