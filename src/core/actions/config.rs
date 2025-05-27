use std::io::{Write, stdout};

use crate::system::{CONFIG_FILE, Configuration};

use super::Command;

enum ConfigOption {
    Print(bool),
}

pub struct ConfigurationCommand<'a> {
    action: ConfigOption,
    configuration: &'a Configuration,
}

impl<'a> Command<'a> for ConfigurationCommand<'a> {
    fn new(
        args: &'a clap::ArgMatches,
        conf: &'a Configuration,
    ) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized,
    {
        Ok(Self {
            action: ConfigOption::Print(
                *args
                    .get_one::<bool>("path")
                    .expect("flag is set to false by default"),
            ),
            configuration: conf,
        })
    }

    fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        // based on current implementation, this match is sufficient.
        // Refactors necessary when extending behaviour.
        let ConfigOption::Print(b) = self.action;
        if b {
            writeln!(stdout(), "{}", CONFIG_FILE)?;
        } else {
            println!(
                r#"notes.toml
    path={}
    editor={}"#,
                self.configuration.settings.path,
                self.configuration
                    .settings
                    .editor
                    .as_ref()
                    .map_or("none", |s| s.as_ref())
            )
        };
        Ok(())
    }
}
