use std::path::{Path, PathBuf};

use crate::{
    cli::{Commands, Notebook},
    core::actions::Command,
};

use super::{
    disallow_files_with_extensions, disallow_operation_on_active_notebook, disallow_reserved_names,
};

pub struct RemoveCommand {
    path: PathBuf,
}

impl Command<'_> for RemoveCommand {
    fn new(
        args: crate::cli::Commands,
        conf: &crate::system::Configuration,
    ) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized,
    {
        let Commands::Notebook {
            notebooks: Some(Notebook::Remove { notebook }),
        } = args
        else {
            unreachable!("Non-notebook remove command passed to remove handler.");
        };
        Ok(Self {
            path: disallow_files_with_extensions(
                Path::new(&conf.settings.path)
                    .parent()
                    .ok_or("unable to get parent path for remove command")?
                    .join(&notebook),
            )
            .and_then(check_dir_exists)
            .and_then(disallow_reserved_names)
            .and_then(disallow_operation_on_active_notebook)?,
        })
    }

    fn execute(self) -> Result<(), Box<dyn std::error::Error>> {
        let mut notes_base = PathBuf::from(
            self.path
                .parent()
                .ok_or("Failed to fetch parent in notebook remove")?
                .parent()
                .ok_or("Failed to fetch parent in notebook remove")?,
        );
        notes_base.push(".notes");
        std::fs::remove_dir_all(&self.path)?;
        Ok(())
    }
}

pub fn check_dir_exists(p: PathBuf) -> Result<PathBuf, Box<dyn std::error::Error>> {
    if !p.try_exists()? {
        return Err(format!(
            "notebook '{}' does not exist",
            p.file_stem()
                .ok_or("check_dir_exist failed for notebook command")?
                .to_string_lossy()
        )
        .into());
    }
    Ok(p)
}
