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
            .ok_or("unable to get path parent in notebook command")?
            .parent()
            .ok_or("unable to get path parent in notebook command")?,
    );
    p.push(".notes");
    let buf = std::fs::read_to_string(p)?;
    println!(
        "{}",
        buf.split_once("notebook: ")
            .ok_or("unable to get notebook from .notes in notebook command")?
            .1
            .trim_end()
    );

    Ok(())
}
