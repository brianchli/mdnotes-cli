mod core;
mod parser;
mod system;

fn main() -> Result<(), ()> {
    let conf = system::notes_init()?;
    let args = parser::cli().get_matches();
    let command = parser::get_command(&args);
    match command {
        Some(("new", args)) => core::create(&conf, args)?,
        Some(("list", args)) => core::list(&conf, args)?,
        _ => core::default(),
    }

    Ok(())
}
