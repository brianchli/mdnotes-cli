use std::io::{Write, stdout};

use crate::system::{CONFIG_FILE, Configuration};

use super::{Command, Commands};

enum ConfigOption {
    Print(bool),
}

pub struct ConfigurationCommand<'a> {
    action: ConfigOption,
    configuration: &'a Configuration,
}

impl<'a> Command<'a> for ConfigurationCommand<'a> {
    fn new(args: Commands, conf: &'a Configuration) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized,
    {
        let Commands::Config { root } = args else {
            unreachable!("Non-configuration command passed to config handler.");
        };
        Ok(Self {
            action: ConfigOption::Print(root),
            configuration: conf,
        })
    }

    fn execute(self) -> Result<(), Box<dyn std::error::Error>> {
        // based on current implementation, this match is sufficient.
        // Refactors necessary when extending behaviour.
        let ConfigOption::Print(b) = self.action;
        if b {
            writeln!(stdout(), "{}", &*CONFIG_FILE)?;
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
