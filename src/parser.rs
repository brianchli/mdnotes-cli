use clap::{ArgMatches, Command};

const EDIT: &str = "new";
const LIST: &str = "lisat";
const MOVE: &str = "mv";
const REMOVE: &str = "rm";

pub fn cli() -> Command {
    Command::new("notes")
        .subcommand_required(true)
        .subcommand(Command::new(EDIT))
        .subcommand(Command::new(LIST))
        .subcommand(Command::new(MOVE))
        .subcommand(Command::new(REMOVE))
}

pub fn get_command() -> Option<(&'static str, ArgMatches)> {
    let args = cli().get_matches();
    match args.subcommand() {
        Some((EDIT, _)) => Some((EDIT, args)),
        Some((LIST, _)) => Some((LIST, args)),
        Some((MOVE, _)) => Some((MOVE, args)),
        Some((REMOVE, _)) => Some((REMOVE, args)),
        _ => unreachable!("invariant: there is always a subcommand specified"),
    }
}
