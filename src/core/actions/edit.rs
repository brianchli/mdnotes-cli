use clap::ArgMatches;

use super::{Command, Configuration};

pub struct Edit {}

impl Command for Edit {
    fn new(_args: &ArgMatches) -> Self {
        todo!()
    }

    fn execute(&self, conf: &Configuration) -> Result<(), ()> {
        todo!()
    }
}
