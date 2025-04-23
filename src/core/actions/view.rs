use clap::ArgMatches;

use super::Action;

pub struct View {}

impl Action for View {
    fn parse(_args: ArgMatches) -> Self {
        todo!()
    }

    fn apply(&self) -> Result<(), ()> {
        todo!()
    }
}

impl View {
    fn new(args: ArgMatches) -> Self {
        View::parse(args)
    }
}

