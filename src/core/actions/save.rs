use std::path::Path;

use chrono::Local;

use crate::system::Configuration;

use super::{Command, Commands};

pub struct SaveCommand<'a> {
    path: &'a Path,
    remote: bool,
}

impl<'a> Command<'a> for SaveCommand<'a> {
    fn new(args: Commands, conf: &'a Configuration) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized,
    {
        let Commands::Save { remote } = args else {
            unreachable!("Non-configuration command passed to config handler.");
        };

        Ok(Self {
            path: Path::new(conf.settings.path.as_str()),
            remote,
        })
    }

    fn execute(self) -> Result<(), Box<dyn std::error::Error>> {
        // check to see if the repository has been initialised
        if !std::path::Path::new(self.path).join(".git").exists() {
            std::process::Command::new("git")
                .args(["-C", self.path.to_str().unwrap(), "init"])
                .status()?;
        }

        std::process::Command::new("git")
            .args(["-C", self.path.to_str().unwrap(), "add", "."])
            .status()?;

        std::process::Command::new("git")
            .args([
                "-C",
                self.path.to_str().unwrap(),
                "commit",
                "-m",
                &format!("update notes: {}", Local::now().to_rfc3339()),
            ])
            .status()?;

        Ok(())
    }
}
