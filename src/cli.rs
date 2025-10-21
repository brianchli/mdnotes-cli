use clap::{Parser, Subcommand, arg};

#[derive(Subcommand, Debug)]
pub(crate) enum Commands {
    #[command(aliases = ["c", "n"], about = "Create a new note or notebook.")]
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

    #[command(alias = "ls", about = "List available notes in notebook.")]
    List {
        #[arg(long, conflicts_with_all = &["full", "short", "category"])]
        root: bool,

        #[arg(long, conflicts_with_all = &["full", "short", "root"])]
        categories: bool,

        #[arg(long, conflicts_with_all = &["full", "short", "root", "categories"])]
        notebooks: bool,

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

    #[command(about = "Switch to a different notebook.")]
    Switch {
        #[arg(long, short, help = "create the notebook if it does not exist")]
        create: bool,
        #[arg(help = "notebook to be switched to")]
        notebook: String,
    },

    #[command(about = "Save notes locally and remotely with git.")]
    Save {
        #[arg(long, help = "Store commits and push to remote repository")]
        remote: bool,
    },

    #[command(about = "Notebook operations and subcommands.")]
    Notebook {
        #[command(subcommand)]
        notebooks: Option<Notebook>,
    },
}

#[derive(Subcommand, Debug)]
pub enum Notebook {
    #[command(about = "Create a notebook")]
    Create { notebook: String },

    #[command(aliases=&["rm"], about = "Remove a notebook")]
    Remove { notebook: String },
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
