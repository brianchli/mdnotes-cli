use clap::ArgMatches;

use super::Command;

pub struct View {}

impl Command for View {
    fn new(args: &ArgMatches) -> Self {
        todo!()
    }

    fn execute(&self, conf: &crate::system::Configuration) -> Result<(), ()> {
        todo!()
    }
}
