use std::{fmt::Write, path::Path};

use crate::{ZygalError, git_info::GitInfo, git_patch::GitPatch};

const PRE_GIT: &str = "%F{0}%K{208} %3(~.*/%1~.%~) ";
const POST_GIT: &str = "\n%F{0}%K{208} %# %f%k ";

pub fn prompt(current_dir: &Path) -> Result<String, ZygalError> {
    let git_segment = make_git_segment(current_dir)?.unwrap_or("".to_string());
    Ok(format!("{PRE_GIT}{git_segment}%f%k{POST_GIT}"))
}

fn make_git_segment(current_dir: &Path) -> Result<Option<String>, ZygalError> {
    let git_info = match GitInfo::from_git_status_output(current_dir) {
        Ok(Some(git_info)) => git_info,
        Ok(None) => return Ok(None),
        Err(e) => return Err(e),
    };
    let git_patch = GitPatch::detect(current_dir);

    let content = if git_info.has_only_branch() && git_patch.is_none() {
        git_info.branch_name
    } else {
        let mut s = format!("{} ", git_info.branch_name);
        if let Some(patch) = git_patch {
            write!(s, "{patch}").map_err(|_| ZygalError::FromatError)?;
        }
        if git_info.unstaged {
            s.push('*');
        }
        if git_info.staged {
            s.push('+');
        }
        if git_info.stash {
            s.push('+');
        }
        if git_info.untracked {
            s.push('%');
        }
        if let Some(remote_diff) = git_info.remote_diff.as_ref() {
            write!(s, "{remote_diff}").map_err(|_| ZygalError::FromatError)?;
        }
        s
    };

    Ok(Some(format!("%K{{220}} [{}] ", shell_escape(&content))))
}

impl GitInfo {
    fn has_only_branch(&self) -> bool {
        !(self.remote_diff.is_some()
            || self.stash
            || self.untracked
            || self.unstaged
            || self.staged)
    }
}

#[cfg(feature = "zsh")]
fn shell_escape(s: &str) -> String {
    s.replace("%", "%%")
}
