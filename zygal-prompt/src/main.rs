use std::env;

use zygal_prompt::{ZygalError, prompt::prompt};

fn main() -> Result<(), ZygalError> {
    let current_dir = env::current_dir().map_err(|_| ZygalError::CurrentDirError)?;
    println!("{}", prompt(&current_dir)?);
    Ok(())
}
