mod config;
mod error;
mod toml;

use std::{env, path::PathBuf};

pub use crate::{
    config::write_config,
    error::{Error, Result},
};

pub fn generate() -> error::Result<()> {
    let config_dst: PathBuf = [&env::var("OUT_DIR").unwrap(), "config.in.rs"]
        .iter()
        .collect();
    write_config(&config_dst, env!("ZYGAL_CONFIG"), env!("ZYGAL_COLORSCHEME"))?;
    println!("cargo::rustc-env=CONFIG_IN={}", config_dst.display());
    Ok(())
}
