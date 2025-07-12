use std::{path::Path, process};

use crate::{ZygalError, git_info::make_git_info};

const PRE_GIT: &str = "%F{0}%K{208} %3(~.*/%1~.%~) ";
const POST_GIT: &str = "\n%F{0}%K{208} %# %f%k ";

pub fn prompt(current_dir: &Path) -> Result<String, ZygalError> {
    let git_segment = if let Some(output) = git_status_output(current_dir)? {
        let git_info = shell_escape(&make_git_info(&output)?);
        format!("%K{{220}} [{git_info}] ")
    } else {
        String::new()
    };

    Ok(format!("{PRE_GIT}{git_segment}%f%k{POST_GIT}"))
}

fn git_status_output(current_dir: &Path) -> Result<Option<String>, ZygalError> {
    let output = process::Command::new("git")
        .args(["status", "--porcelain=v2", "--branch", "--show-stash"])
        .current_dir(current_dir)
        .output()
        .map_err(|_| ZygalError::GitSpawnError)?;

    let stdout = if output.status.success() {
        Some(String::from_utf8(output.stdout).map_err(|_| ZygalError::GitOutputError)?)
    } else {
        None
    };
    Ok(stdout)
}

#[cfg(feature = "zsh")]
fn shell_escape(s: &str) -> String {
    s.replace("%", "%%")
}
