use std::error::Error;

mod core;
mod parser;
mod system;

fn main() -> Result<(), Box<dyn Error>> {
    let conf = system::notes_init()?;
    let args = parser::cli().get_matches();
    let command = parser::get_command(&args);
    if let Err(err) = match command {
        Some(("new", args)) => core::create(&conf, args),
        Some(("list", args)) => core::list(&conf, args),
        Some(("config", args)) => core::config(&conf, args),
        _ => Ok(()),
    } {
        let Some(io_err) = err.downcast_ref::<std::io::Error>() else {
            return Err(err);
        };
        if io_err.kind() == std::io::ErrorKind::BrokenPipe {
            return Ok(());
        }
    };

    Ok(())
}
