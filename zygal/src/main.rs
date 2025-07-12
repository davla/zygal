mod git_info;

use std::{env, path::Path, process};

use zygal::ZygalError;

use crate::git_info::make_git_info;

const PRE_GIT: &str = "%F{0}%K{208} %3(~.*/%1~.%~) ";
const POST_GIT: &str = "\n%F{0}%K{208} %# %f%k ";

fn main() -> Result<(), ZygalError> {
    let git_segment = if let Some(output) = git_status_output()? {
        let git_info = shell_escape(&make_git_info(&output)?);
        format!("%K{{220}} [{git_info}] ")
    } else {
        String::new()
    };

    println!("{PRE_GIT}{git_segment}%f%k{POST_GIT}");
    Ok(())
}

fn git_status_output() -> Result<Option<String>, ZygalError> {
    let output = process::Command::new("git")
        .args(["status", "--porcelain=v2", "--branch", "--show-stash"])
        .output()
        .map_err(|_| ZygalError::GitSpawnError)?;

    let stdout = if output.status.success() {
        Some(String::from_utf8(output.stdout).map_err(|_| ZygalError::GitOutputError)?)
    } else {
        None
    };
    Ok(stdout)
}

fn shell_escape(s: &str) -> String {
    let bin_name = env::var("SHELL").ok().and_then(|shell| {
        Path::new(&shell)
            .file_name()
            .map(|file_name| file_name.to_owned())
    });
    let Some(bin_name) = bin_name else {
        return s.to_string();
    };

    if bin_name == "zsh" {
        s.replace("%", "%%")
    } else {
        s.to_string()
    }
}
