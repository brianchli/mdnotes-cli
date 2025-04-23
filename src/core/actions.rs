mod edit;
use clap::ArgMatches;

use crate::system::Configuration;

pub trait Command {
    /// Flag handling to populate the fields of an action struct
    fn parse_flags(args: ArgMatches) -> Self;
    /// Execute the action
    fn execute(&self, conf: &Configuration) -> Result<(), ()>;
}

pub fn create(conf: &Configuration, args: ArgMatches) -> Result<(), ()> {
    validate_dir(conf, edit::Edit::new(args), false)
}

pub fn list(conf: &Configuration, _args: ArgMatches) -> Result<(), ()> {
    todo!();
}

fn validate_dir<T>(conf: &Configuration, action: T, create: bool) -> Result<(), ()>
where
    T: Command,
{
    // Always create the parent directory if not found.
    if !std::fs::exists(&conf.settings.path).map_err(|_| ())? {
        std::fs::create_dir_all(&conf.settings.path).map_err(|_| ())?;
    };

    // Create the path if create flag is set
    // TODO: Update the input to be the actual file specified by the notes command
    let exists = std::fs::exists(&conf.settings.path).map_err(|_| ())?;
    match (exists, create) {
        (true, false) => {}
        (false, true) => {}
        (true, true) => {}
        (false, false) => {}
    }

    action.execute(conf)
}
