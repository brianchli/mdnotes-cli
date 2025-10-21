use std::path::{Path, PathBuf};

use crate::{
    cli::{Commands, Notebook},
    core::actions::Command,
};

use super::{disallow_operation_on_active_notebook, disallow_reserved_names};

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
        let Commands::Notebook {
            notebooks: Some(Notebook::Create { notebook }),
        } = args
        else {
            unreachable!("Non-notebook create command passed to create handler.");
        };
        Ok(Self {
            path: super::disallow_files_with_extensions(
                Path::new(&conf.settings.path)
                    .parent()
                    .unwrap()
                    .join(&notebook),
            )
            .and_then(disallow_reserved_names)
            .and_then(disallow_operation_on_active_notebook)
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
            "notebook '{}' already exists",
            p.file_name()
                .ok_or("check_dir_exists failed for notebook command")?
                .to_string_lossy()
        )
        .into());
    }
    Ok(p)
}
