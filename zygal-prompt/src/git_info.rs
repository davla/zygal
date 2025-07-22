use std::{fmt::Display, path::Path, process, str::FromStr, sync::LazyLock};

use regex::Regex;

use crate::ZygalError;

#[derive(Debug, PartialEq)]
pub struct GitInfo {
    pub branch_name: String,
    pub remote_diff: Option<GitRemoteDiff>,
    pub stash: bool,
    pub untracked: bool,
    pub staged: bool,
    pub unstaged: bool,
}

#[derive(Debug, PartialEq)]
pub struct GitRemoteDiff {
    pub incoming: bool,
    pub outgoing: bool,
}

static STAGED_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[12u] [MTARCDU].").expect("Static regex is valid"));
static UNSTAGED_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[12u] .[MTARCDU]").expect("Static regex is valid"));

impl FromStr for GitInfo {
    type Err = ZygalError;

    fn from_str(git_status_output: &str) -> Result<Self, Self::Err> {
        let mut lines = git_status_output.lines().map(str::trim);

        let branch_name = Self::make_branch_name(&mut lines)?;

        let lines: Vec<&str> = lines.collect();
        let remote_diff = GitRemoteDiff::parse(lines.iter())?;
        let stash = lines.iter().any(|l| l.starts_with("# stash"));
        let untracked = lines.iter().any(|l| l.starts_with("?"));
        let staged = lines.iter().any(|l| STAGED_REGEX.is_match(l));
        let unstaged = lines.iter().any(|l| UNSTAGED_REGEX.is_match(l));

        Ok(Self {
            branch_name,
            remote_diff,
            stash,
            untracked,
            staged,
            unstaged,
        })
    }
}

impl GitInfo {
    pub fn from_git_status_output(current_dir: &Path) -> Result<Option<Self>, ZygalError> {
        let output = process::Command::new("git")
            .args(["status", "--porcelain=v2", "--branch", "--show-stash"])
            .current_dir(current_dir)
            .output()
            .map_err(|_| ZygalError::GitSpawnError)?;

        if !output.status.success() {
            return Ok(None);
        }

        let stdout = String::from_utf8(output.stdout).map_err(|_| ZygalError::GitOutputError)?;
        stdout.parse::<Self>().map(|git_info| Some(git_info))
    }

    fn make_branch_name<'a>(
        git_status_lines: &mut impl Iterator<Item = &'a str>,
    ) -> Result<String, ZygalError> {
        let sha = git_status_lines.advance()?.split_whitespace().at(2)?;
        let branch_name = git_status_lines.advance()?.split_whitespace().at(2)?;
        Ok(if branch_name == "(detached)" {
            format!("({}...)", &sha[..7])
        } else {
            branch_name.to_string()
        })
    }
}

impl GitRemoteDiff {
    fn parse<'a>(
        mut git_status_lines: impl Iterator<Item = &'a &'a str>,
    ) -> Result<Option<Self>, ZygalError> {
        let Some(ab_line) = git_status_lines.find(|l| l.starts_with("# branch.ab")) else {
            return Ok(None);
        };

        let mut counts = ab_line.split_whitespace().skip(2);

        let outgoing_count = counts
            .advance()?
            .parse::<i64>()
            .map_err(|_| ZygalError::GitOutputError)?;
        let incoming_count = counts
            .advance()?
            .parse::<i64>()
            .map_err(|_| ZygalError::GitOutputError)?;

        Ok(Some(Self {
            incoming: incoming_count != 0,
            outgoing: outgoing_count != 0,
        }))
    }
}

impl Display for GitRemoteDiff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.incoming && !self.outgoing {
            return write!(f, "=");
        }

        if self.incoming {
            write!(f, "<")?;
        }
        if self.outgoing {
            write!(f, ">")?;
        }
        Ok(())
    }
}

trait GitOutputIterator<T> {
    fn advance(&mut self) -> Result<T, ZygalError>;
    fn at(&mut self, at: usize) -> Result<T, ZygalError>;
}

impl<I, T> GitOutputIterator<T> for I
where
    I: Iterator<Item = T>,
{
    fn advance(&mut self) -> Result<T, ZygalError> {
        self.next().ok_or(ZygalError::GitOutputError)
    }

    fn at(&mut self, at: usize) -> Result<T, ZygalError> {
        self.nth(at).ok_or(ZygalError::GitOutputError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_when_no_branch_info() {
        let status_output = "
            # stash number-not-used
        "
        .trim();
        let git_info = status_output.parse::<GitInfo>();
        assert_eq!(git_info, Err(ZygalError::GitOutputError));
    }

    #[test]
    fn uses_branch_name_when_not_detached_head() {
        let branch_name = "feature/honey-badgers";
        let status_output = format!(
            "
            # branch.oid unused-invalid-sha
            # branch.head {branch_name}
            "
        );
        let git_info = status_output.trim().parse::<GitInfo>();
        assert_eq!(
            git_info.map(|info| info.branch_name).as_deref(),
            Ok(branch_name)
        );
    }

    #[test]
    fn uses_commit_sha_when_detached_head() {
        let status_output = "
            # branch.oid faeddf84c5077e7df0025334801d379bb94fc64f
            # branch.head (detached)
        "
        .trim();
        let git_info = status_output.parse::<GitInfo>();
        assert_eq!(
            git_info.map(|info| info.branch_name).as_deref(),
            Ok("(faeddf8...)")
        );
    }

    #[test]
    fn includes_stashes() {
        let status_output = "
            # branch.oid unused-invalid-sha
            # branch.head feature/spider-monkey
            # stash number-not-used
        "
        .trim();
        let git_info = status_output.parse::<GitInfo>();
        assert!(git_info.is_ok_and(|info| info.stash));
    }

    #[test]
    fn includes_untracked() {
        let status_output = "
            # branch.oid unused-invalid-sha
            # branch.head feature/hippopotamus
            ? path-not-used
        "
        .trim();
        let git_info = status_output.parse::<GitInfo>();
        assert!(git_info.is_ok_and(|info| info.untracked))
    }

    #[test]
    fn includes_staged() {
        let status_output = "
            # branch.oid unused-invalid-sha
            # branch.head feature/gorgonopsia
            1 Dwhatever
        "
        .trim();
        let git_info = status_output.parse::<GitInfo>();
        assert!(git_info.is_ok_and(|info| info.staged))
    }

    #[test]
    fn includes_unstaged() {
        let status_output = "
            # branch.oid unused-invalid-sha
            # branch.head feature/microraptoria
            1 WD
        "
        .trim();
        let git_info = status_output.parse::<GitInfo>();
        assert!(git_info.is_ok_and(|info| info.unstaged))
    }

    #[test]
    fn includes_remote_on_par() {
        let status_output = "
            # branch.oid unused-invalid-sha
            # branch.head feature/placoderms
            # branch.ab +0 -0
        "
        .trim();
        let git_info = status_output.parse::<GitInfo>();
        assert!(git_info.is_ok_and(|info| {
            info.remote_diff
                .is_some_and(|remote| !remote.incoming && !remote.outgoing)
        }))
    }

    #[test]
    fn includes_remote_with_outgoing() {
        let status_output = "
            # branch.oid unused-invalid-sha
            # branch.head feature/ichthyosauria
            # branch.ab +7 -0
        "
        .trim();
        let git_info = status_output.parse::<GitInfo>();
        assert!(git_info.is_ok_and(|info| {
            info.remote_diff
                .is_some_and(|remote| remote.outgoing && !remote.incoming)
        }))
    }

    #[test]
    fn includes_remote_with_incoming() {
        let status_output = "
            # branch.oid unused-invalid-sha
            # branch.head feature/hyaenodonta
            # branch.ab +0 -3
        "
        .trim();
        let git_info = status_output.parse::<GitInfo>();
        assert!(git_info.is_ok_and(|info| {
            info.remote_diff
                .is_some_and(|remote| remote.incoming && !remote.outgoing)
        }))
    }

    #[test]
    fn includes_diverged_remote() {
        let status_output = "
            # branch.oid unused-invalid-sha
            # branch.head feature/ceratopsia
            # branch.ab +9 -3
        "
        .trim();
        let git_info = status_output.parse::<GitInfo>();
        assert!(git_info.is_ok_and(|info| {
            info.remote_diff
                .is_some_and(|remote| remote.incoming && remote.outgoing)
        }))
    }

    #[test]
    fn no_remote_diff_when_no_remote() {
        let status_output = "
            # branch.oid unused-invalid-sha
            # branch.head feature/monothremes
            u AD
        "
        .trim();
        let git_info = status_output.parse::<GitInfo>();
        assert!(git_info.is_ok_and(|info| info.remote_diff.is_none()))
    }

    #[test]
    fn includes_branch_staged_untracked_incoming_remote() {
        let status_output = "
            # branch.oid unused-invalid-sha
            # branch.head feature/monothremes
            # branch.ab +0 -12
            2 D.
            1 M.
            ? not-used-file-name
        "
        .trim();
        let git_info = status_output.parse::<GitInfo>();
        assert_eq!(
            git_info,
            Ok(GitInfo {
                branch_name: "feature/monothremes".to_string(),
                staged: true,
                untracked: true,
                remote_diff: Some(GitRemoteDiff {
                    incoming: true,
                    outgoing: false
                }),
                stash: false,
                unstaged: false
            })
        )
    }

    #[test]
    fn includes_sha_stash_unstaged_on_par_remote() {
        let status_output = "
            # branch.oid dfcac0b563036735405591415163566f9f908e1e
            # branch.head (detached)
            # branch.ab +0 -0
            # stash number-not-used
            u .D
            2 .R
        "
        .trim();
        let git_info = status_output.parse::<GitInfo>();
        assert_eq!(
            git_info,
            Ok(GitInfo {
                branch_name: "(dfcac0b...)".to_string(),
                stash: true,
                unstaged: true,
                remote_diff: Some(GitRemoteDiff {
                    incoming: false,
                    outgoing: false
                }),
                staged: false,
                untracked: false,
            })
        )
    }

    #[test]
    fn includes_branch_staged_unstaged_no_remote() {
        let status_output = "
            # branch.oid unused-invalid-sha
            # branch.head feature/hymenoptera
            u .D
            2 D.
            1 DD
        "
        .trim();
        let git_info = status_output.parse::<GitInfo>();
        assert_eq!(
            git_info,
            Ok(GitInfo {
                branch_name: "feature/hymenoptera".to_string(),
                staged: true,
                unstaged: true,
                remote_diff: None,
                stash: false,
                untracked: false,
            })
        )
    }
}
