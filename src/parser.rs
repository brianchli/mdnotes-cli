use clap::{ArgMatches, Command, arg};

const CREATE: &str = "new";
const LIST: &str = "list";
const MOVE: &str = "mv";
const REMOVE: &str = "rm";

pub fn cli() -> Command {
    Command::new("notes")
        .subcommand(
            Command::new(CREATE)
                .alias("n")
                .about("create a new note")
                .args([
                    arg!(-t --tags "set [tags] to created note"),
                    arg!(<name> "name of the note"),
                    arg!([category] "set the category to store the note"),
                ]),
        )
        .subcommand(
            Command::new(LIST).alias("ls").about("list notes").args([
                arg!(-f --full "prints all notes and the contents of each note")
                    .conflicts_with("short"),
                arg!(-s --short "prints summmary of a note in a single line for each note"),
                arg!([category] "category to filter by"),
            ]),
        )
        .subcommand(Command::new(MOVE))
        .subcommand(Command::new(REMOVE))
}

pub fn get_command(args: &ArgMatches) -> Option<(&'static str, &ArgMatches)> {
    match args.subcommand() {
        Some((CREATE, arg)) | Some(("n", arg)) => Some((CREATE, arg)),
        Some((LIST, arg)) | Some(("ls", arg)) => Some((LIST, arg)),
        Some((MOVE, arg)) => Some((MOVE, arg)),
        Some((REMOVE, arg)) => Some((REMOVE, arg)),
        _ => None,
    }
}
