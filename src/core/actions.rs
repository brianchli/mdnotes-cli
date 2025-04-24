mod edit;
mod view;

use crate::system::Configuration;
use clap::ArgMatches;

pub trait Command {
    /// Creates a command using the flags specified to the program
    fn new(args: &ArgMatches, conf: &Configuration) -> Self;
    /// Execute the command
    fn execute(&self) -> Result<(), ()>;
}

pub fn default() {
    todo!("implement default behaviour when no arguments are provided")
}

pub fn create(conf: &Configuration, args: &ArgMatches) -> Result<(), ()> {
    edit::Edit::new(args, conf).execute()
}

pub fn list(conf: &Configuration, args: &ArgMatches) -> Result<(), ()> {
    view::View::new(args, conf).execute()
}
