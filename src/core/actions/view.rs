use std::{
    error::Error,
    fs::DirEntry,
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
};

use clap::ArgMatches;
use termcolor::{Color, ColorSpec, StandardStream, WriteColor};

use crate::system::Configuration;

use super::Command;

macro_rules! write_coloured {

    // bold write
    ($stream: ident, bold, $($arg: tt)+) => {
        $stream.set_color(ColorSpec::new().set_bold(true))?;
        write!($stream, $($arg)+)?;
        $stream.reset()?;
    };

    // write coloured
    ($stream: ident, colour=$colour: expr, $($arg: tt)+) => {
        $stream.set_color(ColorSpec::new().set_fg(Some($colour)))?;
        write!($stream, $($arg)+)?;
        $stream.reset()?;
    };

    // write coloured and bolded
    ($stream: ident, bold_colour=$colour: expr, $($arg: tt)+) => {
        $stream.set_color(ColorSpec::new().set_bold(true).set_fg(Some($colour)))?;
        write!($stream, $($arg)+)?;
        $stream.reset()?;
    };

}

macro_rules! write_colouredln {

    // bold write
    ($stream: ident, bold, $($arg: tt)+) => {
        $stream.set_color(ColorSpec::new().set_bold(true))?;
        writeln!($stream, $($arg)+)?;
        $stream.reset()?;
    };

    // write coloured
    ($stream: ident, colour=$colour: expr, $($arg: tt)+) => {
        $stream.set_color(ColorSpec::new().set_fg(Some($colour)))?;
        writeln!($stream, $($arg)+)?;
        $stream.reset()?;
    };

    // write coloured and bolded
    ($stream: ident, bold_colour=$colour: expr, $($arg: tt)+) => {
        $stream.set_color(ColorSpec::new().set_bold(true).set_fg(Some($colour)))?;
        writeln!($stream, $($arg)+)?;
        $stream.reset()?;
    };

}

#[derive(Debug)]
enum Details {
    Short,
    Full,
    Default,
}

pub struct View {
    path: PathBuf,
    details: Details,
}

impl Command<'_> for View {
    fn new(args: &ArgMatches, conf: &Configuration) -> Result<Self, Box<dyn Error>> {
        // flags are represented as booleans and default to false
        let details = if args.get_one("short").is_some_and(|&v| v) {
            Details::Short
        } else if args.get_one::<bool>("full").is_some_and(|&v| v) {
            Details::Full
        } else {
            Details::Default
        };
        let path = if let Some(cat) = args.get_one::<String>("category") {
            PathBuf::from(format!("{}/{}", conf.settings.path, cat))
        } else {
            PathBuf::from(&conf.settings.path)
        };
        Ok(Self { details, path })
    }

    fn execute(&self) -> Result<(), Box<dyn Error>> {
        match self.details {
            Details::Default => Ok(walk_dir(&self.path, &default_cb)?),
            Details::Full => Ok(walk_dir(&self.path, &full_cb)?),
            Details::Short => {
                let (namelen, taglen) = max_name_and_tag_len(None, &self.path).unwrap();
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
                cb(&entry)?;
            }
        }
    }
    Ok(())
}

/// Computes counts for padding tags and name
fn compute_counts(
    path: &Path,
    mut namelen: usize,
    mut taglen: usize,
) -> Result<(usize, usize), Box<dyn Error>> {
    let reader = BufReader::new(std::fs::File::open(path)?);
    let vals = reader
        .lines()
        .skip(3)
        .map(|v| v.ok().unwrap())
        .collect::<String>();
    let tags = vals
        .split("- ")
        .nth(1)
        .unwrap_or_default()
        .split_once(":")
        .unwrap_or_default()
        .1
        .trim();
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
    let tcount = tags.chars().count();
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
            let (n, c) = max_len.get_or_insert_default();
            (*n, *c) = compute_counts(&path, *n, *c)?;
        }
    }

    Ok(*max_len.get_or_insert_default())
}

fn default_cb(dir: &DirEntry) -> Result<(), Box<dyn Error>> {
    println!("{}", dir.path().to_str().unwrap());
    Ok(())
}

fn full_cb(dir: &DirEntry) -> Result<(), Box<dyn Error>> {
    let reader = BufReader::new(std::fs::File::open(dir.path())?);
    let mut out = StandardStream::stdout(termcolor::ColorChoice::Always);
    write_colouredln!(
        out,
        colour = Color::Green,
        "{}",
        dir.path().to_str().unwrap()
    );
    let mut lines = reader.lines();
    let title = lines.next().ok_or("error")??;
    let header = lines.next().ok_or("error")??;
    for l in lines {
        let st = l?;
        let (header, content) = st
            .split_once("- ")
            .unwrap()
            .1
            .split_once(":")
            .unwrap_or_default();
        let gap = 8 - header.chars().count();
        write_coloured!(out, bold_colour = Color::Yellow, "{}:", header);
        writeln!(
            out,
            "{:>gap$}",
            content,
            gap = if content.chars().count() > gap {
                gap + content.chars().count()
            } else {
                gap
            }
        )?;
    }
    writeln!(out, "\n{}", title)?;
    writeln!(out, "{}\n", header)?;
    Ok(())
}

fn short_cb(dir: &DirEntry, nlen: usize, taglen: usize) -> Result<(), Box<dyn Error>> {
    let reader = BufReader::new(std::fs::File::open(dir.path())?);
    let vals = reader
        .lines()
        .take(5)
        .map(|v| v.ok().unwrap())
        .collect::<Vec<String>>();
    let mut out = StandardStream::stdout(termcolor::ColorChoice::Always);
    let pbuf = dir.path();
    let mut iter = pbuf.iter();
    let file = iter.next_back();
    let category = PathBuf::from_iter(iter.skip_while(|&p| p != "notes").skip(1));
    write_coloured!(
        out,
        colour = Color::Green,
        "/{}",
        category.to_str().unwrap()
    );
    let has_category = !category.to_str().unwrap_or_default().is_empty();
    let gap = nlen - category.to_str().unwrap().chars().count() - has_category as usize;
    if has_category {
        write_coloured!(out, bold, "/");
    }
    write_coloured!(
        out,
        colour = Color::Yellow,
        "{:<gap$}",
        file.unwrap().to_str().unwrap(),
    );
    let gap = taglen + 2;
    write_coloured!(
        out,
        bold,
        " {:<gap$}",
        vals[3].split_once(":").unwrap().1.trim(),
    );

    write_colouredln!(out, bold, "{}", vals[4].split_once(":").unwrap().1.trim(),);
    Ok(())
}
