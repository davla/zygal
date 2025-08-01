use std::{
    env,
    fmt::{self, Write},
    path::{Path, PathBuf},
};

use crate::{git_info::GitInfo, git_patch::GitPatch};

const CURRENT_DIR: &str = "%F{0}%K{208} ";
const NEW_LINE: &str = "\n%F{0}%K{208} %# %f%k ";

pub fn prompt(current_dir: &Path) -> anyhow::Result<String> {
    let current_dir_content = current_dir_segment_content(current_dir);
    let git_segment = if let Some(git_info) = GitInfo::from_git_status_output(current_dir)? {
        let git_patch = GitPatch::detect(current_dir);
        let content = git_segment_content(git_info, git_patch)?;
        format!("%K{{220}} [{}] ", shell_escape(&content))
    } else {
        String::new()
    };

    Ok(format!(
        "{CURRENT_DIR}{current_dir_content} {git_segment}%f%k{NEW_LINE}"
    ))
}

fn current_dir_segment_content(current_dir: &Path) -> String {
    let current_dir = current_dir.home_to_tilde();
    match current_dir.components().count() {
        1 => format!("  {}  ", current_dir.display()),
        2 | 3 => current_dir.display().to_string(),
        _ => format!(
            "*/{}",
            current_dir
                .file_name()
                .expect("Path has at least 3 components, there must be a file name")
                .display()
        ),
    }
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

trait PathExtensions {
    fn home_to_tilde(&self) -> PathBuf;
    fn is_empty(&self) -> bool;
}

impl PathExtensions for Path {
    fn home_to_tilde(&self) -> PathBuf {
        let Some(from_home) = env::home_dir().and_then(|home| self.strip_prefix(&home).ok()) else {
            return self.into();
        };

        let tilde = PathBuf::from("~");
        // Prevent trailing slash
        if from_home.is_empty() {
            tilde
        } else {
            tilde.join(from_home)
        }
    }

    fn is_empty(&self) -> bool {
        self.as_os_str().is_empty()
    }
}

#[cfg(test)]
mod tests {
    use asserting::prelude::*;

    mod git_segment_content {
        use super::super::*;
        use super::*;
        use crate::git_info::GitRemoteDiff;

        #[test]
        fn displays_git_segment_content_in_order() {
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
            let git_segment_content = git_segment_content(git_info, git_patch);
            assert_that(git_segment_content).has_value(format!("{branch} B*+$%="));
        }

        #[test]
        fn does_not_include_trailing_space_if_only_branch_name() {
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
            let git_segment_content = git_segment_content(git_info, git_patch);
            assert_that(git_segment_content).has_value(branch);
        }

        #[test]
        fn skips_remote_symbols_if_no_remote() {
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
            let git_segment_content = git_segment_content(git_info, git_patch);
            assert_that(git_segment_content).has_value(format!("{branch} H+$"));
        }

        #[test]
        fn skips_patch_symbol_if_no_patch() {
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
            let git_segment_content = git_segment_content(git_info, git_patch);
            assert_that(git_segment_content).has_value(format!("{branch} +>"));
        }
    }

    mod current_dir_segment_content {
        use super::super::*;
        use super::*;
        use anyhow::Context;

        #[test]
        fn truncates_from_4th_nested_directory() {
            let current_dir = Path::new("/current/working/directory");
            let current_dir_content = current_dir_segment_content(current_dir);
            assert_that(current_dir_content).is_equal_to("*/directory");
        }

        #[test]
        fn does_not_truncate_until_4th_nested_directory() {
            let current_dir = Path::new("/current/working");
            let current_dir_content = current_dir_segment_content(current_dir);
            assert_that(current_dir_content).is_equal_to("/current/working");
        }

        #[test]
        fn adds_padding_to_root() {
            let current_dir = Path::new("/");
            let current_dir_content = current_dir_segment_content(current_dir);
            assert_that(current_dir_content).is_equal_to("  /  ");
        }

        #[test]
        fn replaces_home_with_tilde() -> anyhow::Result<()> {
            let current_dir = env::home_dir()
                .context("Couldn't retrieve home directory in tests")?
                .join("in/home");
            let current_dir_content = current_dir_segment_content(&current_dir);
            assert_that(current_dir_content).is_equal_to("~/in/home");
            Ok(())
        }

        #[test]
        fn adds_padding_to_home() -> anyhow::Result<()> {
            let current_dir =
                env::home_dir().context("Couldn't retrieve home directory in tests")?;
            let current_dir_content = current_dir_segment_content(&current_dir);
            assert_that(current_dir_content).is_equal_to("  ~  ");
            Ok(())
        }
    }
}
