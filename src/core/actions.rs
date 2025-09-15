mod config;
mod list;
mod create;

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

pub fn create(conf: &Configuration, args: Commands) -> Result<(), Box<dyn Error>> {
    create::CreateCommand::new(args, conf)?.execute()
}

pub fn list(conf: &Configuration, args: Commands) -> Result<(), Box<dyn Error>> {
    list::ListCommand::new(args, conf)?.execute()
}

pub fn config(conf: &Configuration, args: Commands) -> Result<(), Box<dyn Error>> {
    config::ConfigurationCommand::new(args, conf)?.execute()
}
