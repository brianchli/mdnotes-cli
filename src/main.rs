mod cli;
mod core;
mod system;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conf = system::notes_init()?;
    let cli_args = cli::Cli::parse_args();
    if let Err(err) = core::actions::new(&conf, cli_args.commands) {
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
