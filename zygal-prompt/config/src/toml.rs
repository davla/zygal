use std::{fmt, fs, path::PathBuf};

use serde::Deserialize;

use crate::error::{self, ErrExt};

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub shell: Shell,
    pub new_line_content: String,
    pub space_around: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Shell {
    Zsh,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ColorScheme {
    pub current_dir: Colors,
    pub git: Colors,
    pub new_line: Colors,
}

#[derive(Deserialize)]
pub struct Colors {
    #[serde(default)]
    pub background: Color,

    #[serde(default)]
    pub foreground: Color,
}

#[derive(Deserialize, Default)]
pub enum Color {
    #[default]
    #[serde(rename = "reset")]
    Reset,

    #[serde(untagged)]
    AnsiColor(u8),
}

const PACKAGE_DIR: &str = env!("CARGO_MANIFEST_DIR");
const TOML: &str = "toml";

fn read_file_in_config(file_name: &str) -> error::Result<Vec<u8>> {
    let candidates = [
        PathBuf::from(file_name),
        [PACKAGE_DIR, file_name].iter().collect(),
        [PACKAGE_DIR, TOML, file_name].iter().collect(),
        [PACKAGE_DIR, TOML, &format!("{file_name}.toml")]
            .iter()
            .collect(),
    ];

    let file_path = candidates
        .iter()
        .find(|path| path.exists())
        .ok_or_else(|| {
            let paths: Vec<_> = candidates.iter().map(|p| p.display().to_string()).collect();
            error::Error::TomlNotFound {
                file_name: file_name.to_string(),
                candidates: paths,
            }
        })?;

    fs::read(file_path).map_err(|err| err.into())
}

impl Config {
    pub fn read(file_name: &str) -> error::Result<Self> {
        let bytes = read_file_in_config(file_name)?;
        toml::from_slice(&bytes).err_into()
    }
}

impl ColorScheme {
    pub fn read(file_name: &str) -> error::Result<Self> {
        let bytes = read_file_in_config(file_name)?;
        toml::from_slice(&bytes).err_into()
    }
}

impl Shell {
    pub fn foreground_escape(&self, color: &Color) -> String {
        match self {
            Self::Zsh => match color {
                Color::AnsiColor(palette_color) => format!("%F{{{palette_color}}}"),
                Color::Reset => "%f".to_string(),
            },
        }
    }

    pub fn background_escape(&self, color: &Color) -> String {
        match self {
            Self::Zsh => match color {
                Color::AnsiColor(palette_color) => format!("%K{{{palette_color}}}"),
                Color::Reset => "%k".to_string(),
            },
        }
    }

    pub fn reset_escape(&self) -> String {
        self.foreground_escape(&Color::Reset) + &self.background_escape(&Color::Reset)
    }
}

impl fmt::Display for Shell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Zsh => "zsh",
            }
        )
    }
}
