use anyhow::Result;

pub mod counter;
pub mod editor;

fn main() -> Result<()> {
    // counter::startup()?;
    editor::startup()?;
    Ok(())
}
