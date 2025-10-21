mod create;
mod invariants;
mod remove;

use std::path::{Path, PathBuf};

use invariants::*;

pub use create::CreateCommand;
pub use remove::RemoveCommand;

use crate::system::Configuration;

pub fn default(conf: &Configuration) -> Result<(), Box<dyn std::error::Error>> {
    let mut p = PathBuf::from(
        Path::new(&conf.settings.path)
            .parent()
            .ok_or("unable to get path parent in stack command")?
            .parent()
            .ok_or("unable to get path parent in stack command")?,
    );
    p.push(".notes");
    let buf = std::fs::read_to_string(p)?;
    println!(
        "{}",
        buf.split_once("stack: ")
            .ok_or("unable to get stack from .notes in stack command")?
            .1
            .trim_end()
    );

    Ok(())
}
