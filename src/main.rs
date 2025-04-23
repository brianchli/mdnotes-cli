mod core;
mod parser;
pub mod system;

fn main() -> Result<(), ()> {
    let conf = system::notes_init()?;
    let command = parser::get_command();
    match command {
        Some(("new", args)) => core::create(&conf, args)?,
        Some(("list", args)) => core::list(&conf, args)?,
        _ => unreachable!("invariant: a subcommand is always provided"),
    }

    Ok(())
}
