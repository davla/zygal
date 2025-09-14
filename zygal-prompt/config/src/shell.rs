use std::fmt::Display;

use crate::toml_config::{Color, Shell};

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

impl Display for Shell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Zsh => "zsh",
            }
        )
    }
}
