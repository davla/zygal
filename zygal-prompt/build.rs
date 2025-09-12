use std::{
    env,
    fmt::Display,
    fs::{self, File},
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};

use anyhow::Context;
use serde::Deserialize;

const WORKSPACE_DIR: &str = env!("CARGO_MANIFEST_DIR");
const CONFIG_DIR: &str = "config";

fn main() -> anyhow::Result<()> {
    let config: ConfigToml = toml::from_slice(&read_file_in_config(env!("ZYGAL_CONFIG"))?)?;
    let colorscheme: ColorScheme =
        toml::from_slice(&read_file_in_config(env!("ZYGAL_COLORSCHEME"))?)?;

    let config_in_path: PathBuf = [&env::var("OUT_DIR").unwrap(), "config.in.rs"]
        .iter()
        .collect();
    println!("cargo::rustc-env=CONFIG_IN={}", config_in_path.display());

    let config_in = ConfigIn::new(&config, &colorscheme);
    config_in.write(&config_in_path)
}

fn read_file_in_config(file_name: &str) -> anyhow::Result<Vec<u8>> {
    let file_path = [
        PathBuf::from(file_name),
        [WORKSPACE_DIR, CONFIG_DIR, file_name].iter().collect(),
        [WORKSPACE_DIR, CONFIG_DIR, &format!("{file_name}.toml")]
            .iter()
            .collect(),
    ]
    .into_iter()
    .find(|path| path.exists())
    .context(format!("File {file_name} not found"))?;

    fs::read(file_path).map_err(|err| err.into())
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
struct ConfigToml {
    shell: Shell,
    new_line_content: String,
    space_around: bool,
}

#[derive(Copy, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Shell {
    Zsh,
}

impl Shell {
    fn foreground_escape(&self, color: &Color) -> String {
        match self {
            Self::Zsh => match color {
                Color::AnsiColor(palette_color) => format!("%F{{{palette_color}}}"),
                Color::Reset => "%f".to_string(),
            },
        }
    }

    fn background_escape(&self, color: &Color) -> String {
        match self {
            Self::Zsh => match color {
                Color::AnsiColor(palette_color) => format!("%K{{{palette_color}}}"),
                Color::Reset => "%k".to_string(),
            },
        }
    }

    fn reset_escape(&self) -> String {
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

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
struct ColorScheme {
    current_dir: Colors,
    git: Colors,
    new_line: Colors,
}

#[derive(Deserialize)]
struct Colors {
    background: Option<Color>,
    foreground: Option<Color>,
}

#[derive(Deserialize)]
enum Color {
    #[serde(rename = "reset")]
    Reset,

    #[serde(untagged)]
    AnsiColor(u8),
}

struct ConfigIn {
    shell: Shell,
    current_dir: PromptSegmentConfig,
    git: PromptSegmentConfig,
    new_line: String,
}

struct PromptSegmentConfig {
    prefix: String,
    suffix: String,
}

impl ConfigIn {
    fn new(config: &ConfigToml, color_scheme: &ColorScheme) -> Self {
        let padding = if config.space_around {
            " ".to_string()
        } else {
            "".to_string()
        };
        Self {
            shell: config.shell,
            current_dir: PromptSegmentConfig {
                prefix: Self::make_prefix(&config.shell, &color_scheme.current_dir, &padding),
                suffix: padding.clone(),
            },
            git: PromptSegmentConfig {
                prefix: Self::make_prefix(&config.shell, &color_scheme.git, &padding),
                suffix: padding.clone(),
            },
            new_line: format!(
                "{}{}{padding}",
                Self::make_prefix(&config.shell, &color_scheme.new_line, &padding),
                config.new_line_content
            ),
        }
    }

    fn make_prefix(shell: &Shell, colors: &Colors, padding: &str) -> String {
        format!(
            "{}{}{padding}",
            shell.foreground_escape(colors.foreground.as_ref().unwrap_or(&Color::Reset)),
            shell.background_escape(colors.background.as_ref().unwrap_or(&Color::Reset))
        )
    }

    fn write(&self, config_in_path: &Path) -> anyhow::Result<()> {
        let mut w = BufWriter::new(File::create(config_in_path)?);
        write!(
            &mut w,
            include_str!("config/config.in"),
            shell = self.shell,
            reset_style = self.shell.reset_escape(),
            current_dir_prefix = self.current_dir.prefix,
            current_dir_suffix = self.current_dir.suffix,
            git_prefix = self.git.prefix,
            git_suffix = self.git.suffix,
            new_line = self.new_line
        )?;
        w.flush().context(format!(
            "Error while writing {} to disk",
            config_in_path.display()
        ))
    }
}
