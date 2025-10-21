use std::path::{Path, PathBuf};

use crate::{
    cli::{Commands, Stack},
    core::actions::Command,
};

use super::{disallow_operation_on_active_note_stack, disallow_reserved_names};

pub struct CreateCommand {
    path: PathBuf,
}

impl Command<'_> for CreateCommand {
    fn new(
        args: crate::cli::Commands,
        conf: &crate::system::Configuration,
    ) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized,
    {
        let Commands::Stack {
            stack: Some(Stack::Create { stack }),
        } = args
        else {
            unreachable!("Non-stack create command passed to create handler.");
        };
        Ok(Self {
            path: super::disallow_files_with_extensions(
                Path::new(&conf.settings.path)
                    .parent()
                    .unwrap()
                    .join(&stack),
            )
            .and_then(disallow_reserved_names)
            .and_then(disallow_operation_on_active_note_stack)
            .and_then(check_dir_exists)?,
        })
    }

    fn execute(self) -> Result<(), Box<dyn std::error::Error>> {
        std::fs::create_dir_all(self.path)?;
        Ok(())
    }
}

fn check_dir_exists(p: PathBuf) -> Result<PathBuf, Box<dyn std::error::Error>> {
    if p.try_exists()? {
        return Err(format!(
            "note stack '{}' already exists",
            p.file_name()
                .ok_or("check_dir_exists failed for stack command")?
                .to_string_lossy()
        )
        .into());
    }
    Ok(p)
}
