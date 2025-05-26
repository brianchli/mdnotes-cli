use std::error::Error;

mod core;
mod parser;
mod system;

fn main() -> Result<(), Box<dyn Error>> {
    let conf = system::notes_init()?;
    let args = parser::cli().get_matches();
    let command = parser::get_command(&args);
    match command {
        Some(("new", args)) => core::create(&conf, args)?,
        Some(("list", args)) => core::list(&conf, args)?,
        Some(("config", args)) => core::config(&conf, args)?,
        _ => {}
    }
    Ok(())
}
