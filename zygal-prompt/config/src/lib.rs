mod error;
mod shell;
mod toml_config;

use std::{
    env,
    fs::File,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};

pub use error::{Error, Result};

use crate::error::ErrExt;

pub fn generate() -> error::Result<()> {
    let config = toml_config::Config::read(env!("ZYGAL_CONFIG"))?;
    let color_scheme = toml_config::ColorScheme::read(env!("ZYGAL_COLORSCHEME"))?;

    let config_in_path: PathBuf = [&env::var("OUT_DIR").unwrap(), "config.in.rs"]
        .iter()
        .collect();
    println!("cargo::rustc-env=CONFIG_IN={}", config_in_path.display());

    write_config_in(&config_in_path, &config, &color_scheme)
}

fn write_config_in(
    dst_path: &Path,
    config: &toml_config::Config,
    color_scheme: &toml_config::ColorScheme,
) -> error::Result<()> {
    let mut writer = BufWriter::new(File::create(dst_path)?);

    let padding = if config.space_around {
        " ".to_string()
    } else {
        "".to_string()
    };

    let new_line = format!(
        "{}{}{padding}",
        make_prefix(&config.shell, &color_scheme.new_line, &padding),
        config.new_line_content
    );

    write!(
        &mut writer,
        include_str!("config.in"),
        shell = config.shell,
        reset_style = config.shell.reset_escape(),
        current_dir_prefix = make_prefix(&config.shell, &color_scheme.current_dir, &padding),
        current_dir_suffix = padding.clone(),
        git_prefix = make_prefix(&config.shell, &color_scheme.git, &padding),
        git_suffix = padding.clone(),
        new_line = new_line
    )?;
    writer.flush().err_into()
}

fn make_prefix(shell: &toml_config::Shell, colors: &toml_config::Colors, padding: &str) -> String {
    format!(
        "{}{}{padding}",
        shell.foreground_escape(&colors.foreground),
        shell.background_escape(&colors.background)
    )
}
