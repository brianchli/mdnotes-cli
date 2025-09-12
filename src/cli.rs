use clap::{Parser, Subcommand, arg};

#[derive(Subcommand, Debug)]
pub(crate) enum Commands {
    #[command(alias = "n", about = "Create a new note.")]
    Create {
        #[arg(long, help = "create a new note without opening for editing")]
        quiet: bool,
        #[arg(short, long, help = "set the category for the new note")]
        category: Option<String>,
        #[arg(help = "the title of the new note")]
        name: String,
        #[arg(help = "keywords to be associated with a note")]
        tags: Option<Vec<String>>,
    },

    #[command(alias = "ls", about = "List available notes.")]
    List {
        #[arg(long, conflicts_with_all = &["full", "short", "category"])]
        root: bool,

        #[arg(
            short,
            long,
            conflicts_with = "short",
            help = "prints all notes and the content of each note"
        )]
        full: bool,

        #[arg(
            short,
            long,
            conflicts_with = "full",
            help = "prints summmary of a note in a single line for each note"
        )]
        short: bool,

        #[arg(help = "category to filter by")]
        category: Option<String>,
    },

    #[command(about = "Notes configuration.")]
    Config {
        #[arg(help = "prints the notes configuration directory")]
        root: bool,
    },
}

#[derive(Parser, Debug)]
#[command(name = "notes", about = "Create markdown notes in the terminal.")]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) commands: Commands,
}
