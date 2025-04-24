use clap::ArgMatches;

use crate::system::Configuration;

use super::Command;

pub struct View {}

impl Command for View {
    fn new(args: &ArgMatches, conf: &Configuration) -> Self {
        todo!()
    }

    fn execute(&self) -> Result<(), ()> {
        todo!()
    }
}
