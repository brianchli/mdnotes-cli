use clap::{Arg, ArgAction, Command};
use clap_complete::aot::Zsh;
use clap_complete::generate_to;
use std::io::Error;
use std::path::PathBuf;

pub fn build_cli() -> Command {
    Command::new("notes")
        .about("Create markdown notes in the terminal.")
        // Subcommands
        .subcommand(
            Command::new("create")
                .about("Create a new note.")
                .aliases(["c", "n"])
                .arg(
                    Arg::new("quiet")
                        .long("quiet")
                        .help("create a new note without opening for editing")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("category")
                        .short('c')
                        .long("category")
                        .help("set the category for the new note")
                        .num_args(1),
                )
                .arg(
                    Arg::new("name")
                        .help("the title of the new note")
                        .required(true),
                )
                .arg(
                    Arg::new("tags")
                        .help("keywords to be associated with a note")
                        .num_args(0..), // optional multiple values
                ),
        )
        .subcommand(
            Command::new("list")
                .about("List available notes in note stack.")
                .alias("ls")
                .arg(
                    Arg::new("root")
                        .long("root")
                        .conflicts_with_all(["full", "short", "category"])
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("categories")
                        .long("categories")
                        .conflicts_with_all(["full", "short", "root"])
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("stacks")
                        .long("stacks")
                        .conflicts_with_all(["full", "short", "root", "categories"])
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("full")
                        .short('f')
                        .long("full")
                        .conflicts_with("short")
                        .help("prints all notes and the content of each note")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("short")
                        .short('s')
                        .long("short")
                        .conflicts_with("full")
                        .help("prints summary of a note in a single line for each note")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("category")
                        .help("match string to filter by")
                        .num_args(1),
                ),
        )
        .subcommand(
            Command::new("config").about("Notes configuration.").arg(
                Arg::new("path")
                    .long("path")
                    .help("prints the notes configuration directory")
                    .action(ArgAction::SetTrue),
            ),
        )
        .subcommand(
            Command::new("switch")
                .about("Switch to a different note stack.")
                .arg(
                    Arg::new("create")
                        .short('c')
                        .long("create")
                        .help("create the note stack if it does not exist")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("stack")
                        .help("note stack to be switched to")
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("save")
                .about("Save notes locally and remotely with git.")
                .arg(
                    Arg::new("remote")
                        .long("remote")
                        .help("Store commits and push to remote repository")
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("stack")
                .about("Note stack operations and subcommands.")
                .subcommand(
                    Command::new("create")
                        .about("Create a notes stack")
                        .arg(Arg::new("stack").help("name of the stack").required(true)),
                )
                .subcommand(
                    Command::new("remove")
                        .about("Remove a notes stack")
                        .aliases(["rm"])
                        .arg(
                            Arg::new("stack")
                                .help("name of the stack to remove")
                                .required(true),
                        ),
                ),
        )
}

fn main() -> Result<(), Error> {
    if let Ok(env) = std::env::var("RUST_BUILD") {
        if env != "release" {
            println!("cargo:warning=completion file generation omitted");
            return Ok(());
        }
    } else {
        println!("cargo:warning=completion file generation omitted");
        return Ok(());
    };

    let outdir = PathBuf::from("/Users/brianli/Documents/projects/rs/notes");
    let mut cmd = build_cli();
    let path = generate_to(
        Zsh, &mut cmd, // We need to specify what generator to use
        "notes",  // We need to specify the bin name manually
        outdir,   // We need to specify where to write to
    )?;

    println!("cargo:warning=completion file is generated: {path:?}");

    Ok(())
}
