use super::ListEntry;
use chrono::{DateTime, Local};
use std::{
    collections::BinaryHeap,
    error::Error,
    io::{BufRead, IsTerminal, Write},
};

use termcolor::{Color, StandardStream, WriteColor};

use crate::{
    core::markdown::NotesFrontMatter,
    system::{self},
    write_coloured, write_colouredln,
};

/// List all paths as an absolute path.
pub fn default(mut entries: BinaryHeap<ListEntry>) -> Result<(), Box<dyn Error>> {
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
                    .split(&*system::DATA_DIR)
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

/// List all paths and content
pub fn full(mut entries: BinaryHeap<ListEntry>) -> Result<(), Box<dyn Error>> {
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
                    .split(&*system::DATA_DIR)
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

/// List all files and relevant metadata
pub fn short(
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

        let mut iter = entry.path.iter();
        let file = iter.next_back();

        let mut gap = nlen;
        if let Some(category) = &notes_metadata.category {
            gap = nlen - category.chars().count();
            write_coloured!(out, colour = Color::Green, "/{}", category);
            if let Some(subcategories) = &notes_metadata.subcategories {
                for s in subcategories {
                    write_coloured!(out, colour = Color::Green, "/{}", s);
                    gap -= s.chars().count() + 1;
                }
            }
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
        write_colouredln!(out, bold, "{}", dt.format("%d-%b-%Y %H:%M:%S %P %z"));
    }
    Ok(())
}
