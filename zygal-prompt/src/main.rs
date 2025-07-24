use std::env;

use anyhow::Context;
use zygal_prompt::prompt::prompt;

fn main() -> anyhow::Result<()> {
    let current_dir = env::current_dir().context("Failed retrieving current working directory")?;
    println!("{}", prompt(&current_dir)?);
    Ok(())
}
