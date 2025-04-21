mod edit;
mod system;

fn main() -> Result<(), ()> {
    system::notes_init()?;

    Ok(())
}
