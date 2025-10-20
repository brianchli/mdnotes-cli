use std::path::{Path, PathBuf};

use super::{Command, Commands};

pub struct SwitchCommand<'a> {
    path: &'a Path,
    notebook: String,
    create: bool,
}

impl<'a> Command<'a> for SwitchCommand<'a> {
    fn new(
        args: Commands,
        conf: &'a crate::system::Configuration,
    ) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized,
    {
        let Commands::Switch { notebook, create } = args else {
            unreachable!("Non-switch command passed to switch handler.");
        };

        Ok(Self {
            path: Path::new(&conf.settings.path),
            notebook,
            create,
        })
    }

    fn execute(self) -> Result<(), Box<dyn std::error::Error>> {
        let p = Path::new(&self.path);
        let mut notes_base = PathBuf::from(p.parent().unwrap().parent().unwrap());
        let mut notebook_path = PathBuf::from(p.parent().unwrap());

        notes_base.push(".notes");
        notebook_path.push(&self.notebook);

        if !self.create && !notebook_path.try_exists()? {
            return Err(format!("invalid notebook '{}'", self.notebook).into());
        } else if self.create {
            std::fs::create_dir_all(&notebook_path)?;
        }

        let buf = std::fs::read_to_string(&notes_base)?;
        if buf.split_once("notebook: ").unwrap().1 == self.notebook {
            println!("already in notebook '{}'", self.notebook);
            return Ok(());
        }

        std::fs::write(&notes_base, format!("notebook: {}", self.notebook))?;
        Ok(())
    }
}
