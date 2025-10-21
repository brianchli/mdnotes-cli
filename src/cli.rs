use clap::{Parser, Subcommand, arg};

#[derive(Subcommand, Debug)]
pub(crate) enum Commands {
    #[command(aliases = ["c", "n"], about = "Create a new note.")]
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

    #[command(alias = "ls", about = "List available notes in note stack.")]
    List {
        #[arg(long, conflicts_with_all = &["full", "short", "category"])]
        root: bool,

        #[arg(long, conflicts_with_all = &["full", "short", "root"])]
        categories: bool,

        #[arg(long, conflicts_with_all = &["full", "short", "root", "categories"])]
        stacks: bool,

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

        #[arg(help = "match string to filter by")]
        category: Option<String>,
    },

    #[command(about = "Notes configuration.")]
    Config {
        #[arg(long, help = "prints the notes configuration directory")]
        path: bool,
    },

    #[command(about = "Switch to a different note stack.")]
    Switch {
        #[arg(long, short, help = "create the note stack if it does not exist")]
        create: bool,
        #[arg(help = "note stack to be switched to")]
        stack: String,
    },

    #[command(about = "Save notes locally and remotely with git.")]
    Save {
        #[arg(long, help = "Store commits and push to remote repository")]
        remote: bool,
    },

    #[command(about = "Note stack operations and subcommands.")]
    Stack {
        #[command(subcommand)]
        stack: Option<Stack>,
    },
}

#[derive(Subcommand, Debug)]
pub enum Stack {
    #[command(about = "Create a notes stack")]
    Create { stack: String },

    #[command(aliases=&["rm"], about = "Remove a notes stack")]
    Remove { stack: String },
}

#[derive(Parser, Debug)]
#[command(name = "notes", about = "Create markdown notes in the terminal.")]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) commands: Commands,
}

impl Cli {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
