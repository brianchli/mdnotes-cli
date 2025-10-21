use std::path::{Path, PathBuf};

use super::{Command, Commands};

pub struct SwitchCommand<'a> {
    path: &'a Path,
    stack: String,
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
        let Commands::Switch { stack, create } = args else {
            unreachable!("Non-switch command passed to switch handler.");
        };

        Ok(Self {
            path: Path::new(&conf.settings.path),
            stack,
            create,
        })
    }

    fn execute(self) -> Result<(), Box<dyn std::error::Error>> {
        let p = Path::new(&self.path);
        let mut notes_base = PathBuf::from(p.parent().unwrap().parent().unwrap());
        let mut stack_path = PathBuf::from(p.parent().unwrap());

        notes_base.push(".notes");
        stack_path.push(&self.stack);

        if !self.create && !stack_path.try_exists()? {
            return Err(format!("invalid note stack '{}'", self.stack).into());
        } else if self.create {
            std::fs::create_dir_all(&stack_path)?;
        }

        let buf = std::fs::read_to_string(&notes_base)?;
        if buf.split_once("stack: ").unwrap().1 == self.stack {
            println!("already in note stack '{}'", self.stack);
            return Ok(());
        }

        std::fs::write(&notes_base, format!("stack: {}", self.stack))?;
        println!("switched to note stack '{}'", self.stack);
        Ok(())
    }
}
