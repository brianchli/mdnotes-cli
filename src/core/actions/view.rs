use std::{error::Error, fs::DirEntry, ops::Deref, path::Path};

use clap::ArgMatches;

use crate::system::Configuration;

use super::Command;

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
        let details = if args.get_one("short").is_some_and(|v| *v) {
            Details::Short
        } else if args.get_one::<bool>("full").is_some_and(|v| *v) {
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
        if let Details::Default = self.details {
            walk_dir(path, default_cb)?
        } else {
            unimplemented!("full and short flags are not implemented")
        }
        Ok(())
    }
}

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
