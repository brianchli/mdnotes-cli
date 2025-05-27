mod new;
mod list;
mod config;

use std::error::Error;

use crate::system::Configuration;
use clap::ArgMatches;

pub trait Command<'a> {
    /// Creates a command using the flags specified to the program
    fn new(args: &'a ArgMatches, conf: &'a Configuration) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;
    /// Execute the command
    fn execute(&self) -> Result<(), Box<dyn Error>>;
}

pub fn create(conf: &Configuration, args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    new::NewCommand::new(args, conf)?.execute()
}

pub fn list(conf: &Configuration, args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    list::ListCommand::new(args, conf)?.execute()
}

pub fn config(conf: &Configuration, args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    config::ConfigurationCommand::new(args, conf)?.execute()
}
