mod git_info;

use std::process;

use zygal::ZygalError;

use crate::git_info::make_git_info;

const PRE_GIT: &str = "%F{0}%K{208} %3(~.*/%1~.%~) ";
const POST_GIT: &str = "\n%F{0}%K{208} %# %f%k ";

fn main() -> Result<(), ZygalError> {
    let git_segment = if let Some(output) = git_status_output()? {
        format!("%K{{220}} {} ", make_git_info(&output)?)
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
