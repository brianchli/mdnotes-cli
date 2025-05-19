use std::{error::Error, fs::DirEntry, ops::Deref, path::Path};

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
    path: String,
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
            format!("{}/{}", conf.settings.path, cat)
        } else {
            conf.settings.path.clone()
        };
        Ok(Self { details, path })
    }

    fn execute(&self) -> Result<(), Box<dyn Error>> {
        let path = Path::new(self.path.deref());
        match self.details {
            Details::Default => Ok(walk_dir(path, default_cb)?),
            _ => unimplemented!("full and short flags are not implemented"),
        }
    }
}

/// Walks the directory and applies the callback function.
///
/// code snippet taken from std::fs::read_dir documentation.
fn walk_dir(dir: &Path, cb: fn(&DirEntry)) -> std::io::Result<()> {
    if dir.is_dir() {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                walk_dir(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

fn default_cb(dir: &DirEntry) {
    println!("{}", dir.path().to_str().unwrap());
}
