mod edit;
mod view;

use clap::ArgMatches;
use crate::system::Configuration;

pub trait Command {
    /// Creates a command using the flags specified to the program
    fn new(args: &ArgMatches) -> Self;
    /// Execute the command
    fn execute(&self, conf: &Configuration) -> Result<(), ()>;
}

pub fn default() {
    todo!("implement default behaviour when no arguments are provided")
}

pub fn create(conf: &Configuration, args: &ArgMatches) -> Result<(), ()> {
    edit::Edit::new(args).execute(conf)
}

pub fn list(conf: &Configuration, args: &ArgMatches) -> Result<(), ()> {
    view::View::new(args).execute(conf)
}
