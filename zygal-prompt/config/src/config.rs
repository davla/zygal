use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use crate::error::{self, ErrExt};

pub fn write_config(
    dst_path: &Path,
    config_file_name: &str,
    color_scheme_file_name: &str,
) -> error::Result<()> {
    let mut writer = BufWriter::new(File::create(dst_path)?);
    let config = crate::toml::Config::read(config_file_name)?;
    let color_scheme = crate::toml::ColorScheme::read(color_scheme_file_name)?;

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
        new_line = new_line,
        git_merge = make_optional_string(&config.git.merge),
        git_rebase = make_optional_string(&config.git.rebase),
        git_cherry_pick = make_optional_string(&config.git.cherry_pick),
        git_revert = make_optional_string(&config.git.revert),
        git_unstaged = make_optional_string(&config.git.unstaged),
        git_staged = make_optional_string(&config.git.staged),
        git_stash = make_optional_string(&config.git.stash),
        git_untracked = make_optional_string(&config.git.untracked),
        git_remote = format!("{:?}", &config.git.remote)
    )?;
    writer.flush().err_into()
}

fn make_prefix(shell: &crate::toml::Shell, colors: &crate::toml::Colors, padding: &str) -> String {
    format!(
        "{}{}{padding}",
        shell.foreground_escape(&colors.foreground),
        shell.background_escape(&colors.background)
    )
}

fn make_optional_string(s: &Option<String>) -> String {
    format!("{:?}", s.as_ref().filter(|text| !text.is_empty()))
}
