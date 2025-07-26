use std::{
    fmt::{self, Write},
    path::Path,
};

use crate::{git_info::GitInfo, git_patch::GitPatch};

const PRE_GIT: &str = "%F{0}%K{208} %3(~.*/%1~.%~) ";
const POST_GIT: &str = "\n%F{0}%K{208} %# %f%k ";

pub fn prompt(current_dir: &Path) -> anyhow::Result<String> {
    let git_segment = if let Some(git_info) = GitInfo::from_git_status_output(current_dir)? {
        let git_patch = GitPatch::detect(current_dir);
        let content = git_segment_content(git_info, git_patch)?;
        format!("%K{{220}} [{}] ", shell_escape(&content))
    } else {
        String::new()
    };

    Ok(format!("{PRE_GIT}{git_segment}%f%k{POST_GIT}"))
}

fn git_segment_content(
    git_info: GitInfo,
    git_patch: Option<GitPatch>,
) -> Result<String, fmt::Error> {
    if git_info.has_only_branch() && git_patch.is_none() {
        return Ok(git_info.branch_name);
    }

    let mut s = git_info.branch_name;
    s.push(' ');
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
        s.push('$');
    }
    if git_info.untracked {
        s.push('%');
    }
    if let Some(remote_diff) = git_info.remote_diff.as_ref() {
        write!(s, "{remote_diff}")?;
    }
    Ok(s)
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

#[cfg(test)]
mod tests {
    use crate::git_info::GitRemoteDiff;

    use super::*;

    #[test]
    fn displays_git_segment_content_in_order() -> anyhow::Result<()> {
        let branch = "feature/theropods";
        let git_info = GitInfo {
            branch_name: branch.to_string(),
            remote_diff: Some(GitRemoteDiff {
                incoming: false,
                outgoing: false,
            }),
            stash: true,
            untracked: true,
            staged: true,
            unstaged: true,
        };
        let git_patch = Some(GitPatch::Rebase);
        let git_segment_content = git_segment_content(git_info, git_patch)?;
        assert_eq!(git_segment_content, format!("{branch} B*+$%="));
        Ok(())
    }

    #[test]
    fn does_not_include_trailing_space_if_only_branch_name() -> anyhow::Result<()> {
        let branch = "feature/mobula";
        let git_info = GitInfo {
            branch_name: branch.to_string(),
            remote_diff: None,
            stash: false,
            untracked: false,
            staged: false,
            unstaged: false,
        };
        let git_patch = None;
        let git_segment_content = git_segment_content(git_info, git_patch)?;
        assert_eq!(git_segment_content, branch);
        Ok(())
    }

    #[test]
    fn skips_remote_symbols_if_no_remote() -> anyhow::Result<()> {
        let branch = "feature/mellivora";
        let git_info = GitInfo {
            branch_name: branch.to_string(),
            remote_diff: None,
            stash: true,
            untracked: false,
            staged: true,
            unstaged: false,
        };
        let git_patch = Some(GitPatch::CherryPick);
        let git_segment_content = git_segment_content(git_info, git_patch)?;
        assert_eq!(git_segment_content, format!("{branch} H+$"));
        Ok(())
    }

    #[test]
    fn skips_patch_symbol_if_no_patch() -> anyhow::Result<()> {
        let git_patch = None;
        let branch = "feature/ateles";
        let git_info = GitInfo {
            branch_name: branch.to_string(),
            remote_diff: Some(GitRemoteDiff {
                incoming: false,
                outgoing: true,
            }),
            stash: false,
            untracked: false,
            staged: true,
            unstaged: false,
        };
        let git_segment_content = git_segment_content(git_info, git_patch)?;
        assert_eq!(git_segment_content, format!("{branch} +>"));
        Ok(())
    }
}
