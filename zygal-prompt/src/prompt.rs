use std::{fmt::Write, path::Path};

use crate::{git_info::GitInfo, git_patch::GitPatch};

const PRE_GIT: &str = "%F{0}%K{208} %3(~.*/%1~.%~) ";
const POST_GIT: &str = "\n%F{0}%K{208} %# %f%k ";

pub fn prompt(current_dir: &Path) -> anyhow::Result<String> {
    let git_segment = make_git_segment(current_dir)?.unwrap_or("".to_string());
    Ok(format!("{PRE_GIT}{git_segment}%f%k{POST_GIT}"))
}

fn make_git_segment(current_dir: &Path) -> anyhow::Result<Option<String>> {
    let Some(git_info) = GitInfo::from_git_status_output(current_dir)? else {
        return Ok(None);
    };
    let git_patch = GitPatch::detect(current_dir);

    let content = if git_info.has_only_branch() && git_patch.is_none() {
        git_info.branch_name
    } else {
        let mut s = format!("{} ", git_info.branch_name);
        if let Some(patch) = git_patch {
            write!(s, "{patch}")?;
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
            write!(s, "{remote_diff}")?;
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
