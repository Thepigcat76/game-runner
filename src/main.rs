use std::process::Command;

use anyhow::{bail, Context};

fn main() -> anyhow::Result<()> {
    let home_dir = dirs::home_dir()
        .context("Failed to get home dir")?
        .join("game");
    let status = Command::new("python")
        .arg(home_dir.join("main.py"))
        .status()?;
    if status.success() {
        return Ok(())
    }
    bail!("Failed to run program")
}
