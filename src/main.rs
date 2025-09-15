use clap::Parser;
use std::error::Error;

mod cli;
mod core;
mod system;

fn main() -> Result<(), Box<dyn Error>> {
    let conf = system::notes_init()?;
    let cli_args = cli::Cli::parse();

    if let Err(err) = match &cli_args.commands {
        cli::Commands::Create { .. } => core::create(&conf, cli_args.commands),
        cli::Commands::List { .. } => core::list(&conf, cli_args.commands),
        cli::Commands::Config { .. } => core::config(&conf, cli_args.commands),
    } {
        // handle broken pipe errors
        return if let Some(io_err) = err.downcast_ref::<std::io::Error>() {
            if io_err.kind() == std::io::ErrorKind::BrokenPipe {
                Ok(())
            } else {
                Err(err)
            }
        } else {
            Err(err)
        };
    };

    Ok(())
}
