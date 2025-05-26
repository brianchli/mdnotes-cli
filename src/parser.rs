use clap::{ArgMatches, Command, arg};

const CREATE: &str = "new";
const LIST: &str = "list";
const CONFIG: &str = "config";

pub fn cli() -> Command {
    Command::new("notes")
        .arg_required_else_help(true)
        .about("Create markdown notes in the terminal")
        .subcommand(
            Command::new(CREATE)
                .alias("n")
                .about("create a new note")
                .args([
                    arg!(--quiet "silently create a note"),
                    arg!(-c --category <value> "set the category to store the note"),
                    arg!(<name> "name of the note"),
                    arg!([tags] "set [tags] for the created note").num_args(0..10),
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
        .subcommand(
            Command::new(CONFIG)
                .about("notes configuration")
                .args([arg!(--root "only print the notes root directory")]),
        )
}

pub fn get_command(args: &ArgMatches) -> Option<(&'static str, &ArgMatches)> {
    match args.subcommand() {
        Some((CREATE, arg)) | Some(("n", arg)) => Some((CREATE, arg)),
        Some((LIST, arg)) | Some(("ls", arg)) => Some((LIST, arg)),
        Some((CONFIG, arg)) => Some((CONFIG, arg)),
        _ => None,
    }
}
