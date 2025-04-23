use super::{Command, Configuration};

use crate::system::DATA_DIR as default_directory;

pub struct Edit {}

impl Command for Edit {
    fn parse_flags(_args: clap::ArgMatches) -> Self {
        todo!()
    }

    fn execute(&self, conf: &Configuration) -> Result<(), ()> {
        todo!()
    }
}

impl Edit {
    pub fn new(args: clap::ArgMatches) -> Self {
        Edit::parse_flags(args)
    }
}
