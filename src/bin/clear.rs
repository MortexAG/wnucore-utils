use crossterm::{execute, terminal::{Clear, ClearType}};
use std::io::{stdout, Result};

fn main() -> Result<()> {
    execute!(stdout(), Clear(ClearType::All))?;
    Ok(())
}
