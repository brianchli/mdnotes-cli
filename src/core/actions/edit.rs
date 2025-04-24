use clap::ArgMatches;

use super::{Command, Configuration};

pub struct Edit {}

impl Command for Edit {
    fn new(_args: &ArgMatches, conf: &Configuration) -> Self {
        todo!()
    }

    fn execute(&self) -> Result<(), ()> {
        Ok(())
    }
}
