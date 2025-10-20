mod config;
mod create;
mod list;
mod notebook;
mod save;
mod switch;

use std::error::Error;

use crate::{cli::Commands, system::Configuration};

pub trait Command<'a> {
    /// Creates a command using the flags specified to the program
    fn new(args: Commands, conf: &'a Configuration) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;
    /// Execute the command
    fn execute(self) -> Result<(), Box<dyn Error>>;
}

pub fn new(conf: &Configuration, args: Commands) -> Result<(), Box<dyn Error>> {
    match args {
        Commands::Create { .. } => create::CreateCommand::new(args, conf)?.execute(),
        Commands::List { .. } => list::ListCommand::new(args, conf)?.execute(),
        Commands::Config { .. } => config::ConfigurationCommand::new(args, conf)?.execute(),
        Commands::Switch { .. } => switch::SwitchCommand::new(args, conf)?.execute(),
        Commands::Save { .. } => save::SaveCommand::new(args, conf)?.execute(),
        Commands::Notebook { ref notebooks } => match notebooks {
            crate::cli::Notebook::Create { .. } => {
                notebook::CreateCommand::new(args, conf)?.execute()
            }
            crate::cli::Notebook::Remove { .. } => {
                notebook::RemoveCommand::new(args, conf)?.execute()
            }
        },
    }
}
