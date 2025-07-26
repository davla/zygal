use std::{fmt::Display, path::Path, process, str::FromStr, sync::LazyLock};

use anyhow::Context;
use regex::Regex;

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
    type Err = anyhow::Error;

    fn from_str(git_status_output: &str) -> anyhow::Result<Self> {
        let mut lines = git_status_output.lines().map(str::trim);

        let branch_name = Self::make_branch_name(&mut lines)?;

        let lines: Vec<&str> = lines.collect();
        let remote_diff = GitRemoteDiff::parse(lines.iter().copied())?;
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
    pub fn from_git_status_output(current_dir: &Path) -> anyhow::Result<Option<Self>> {
        let output = process::Command::new("git")
            .args(["status", "--porcelain=v2", "--branch", "--show-stash"])
            .current_dir(current_dir)
            .output()
            .context("Failed to spawn subprocess to execute git status")?;

        if !output.status.success() {
            return Ok(None);
        }

        let stdout = String::from_utf8(output.stdout)
            .context("Failed to read git status output as an UTF-8 string")?;
        stdout.parse::<Self>().map(Some)
    }

    fn make_branch_name<'a>(
        git_status_lines: &mut impl Iterator<Item = &'a str>,
    ) -> anyhow::Result<String> {
        let sha = git_status_lines
            .next()
            .context("Missing # branch.oid header in git status output")?
            .split_whitespace()
            .nth(2)
            .context("Malformed # branch.oid header in git status output")?;

        let branch_name = git_status_lines
            .next()
            .context("Missing # branch.head header in git status output")?
            .split_whitespace()
            .nth(2)
            .context("Malformed # branch.head header in git status output")?;

        Ok(if branch_name == "(detached)" {
            format!("({}...)", &sha[..7])
        } else {
            branch_name.to_string()
        })
    }
}

impl GitRemoteDiff {
    fn parse<'a>(
        mut git_status_lines: impl Iterator<Item = &'a str>,
    ) -> anyhow::Result<Option<Self>> {
        let Some(ab_line) = git_status_lines.find(|l| l.starts_with("# branch.ab")) else {
            return Ok(None);
        };

        let mut counts = ab_line.split_whitespace().skip(2);
        let outgoing_count = counts
            .next()
            .context("Missing outgoing count in git status # branch.ab header")?
            .parse::<i64>()
            .context("Failed to parse outgoing count in git status # branch.ab header")?;
        let incoming_count = counts
            .next()
            .context("Missing incoming count in git status # branch.ab header")?
            .parse::<i64>()
            .context("Failed to parse incoming count in git status # branch.ab header")?;

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
        assert!(git_info.is_err());
    }

    #[test]
    fn uses_branch_name_when_not_detached_head() -> anyhow::Result<()> {
        let branch_name = "feature/honey-badgers";
        let status_output = format!(
            "
            # branch.oid unused-invalid-sha
            # branch.head {branch_name}
            "
        );
        let git_info = status_output.trim().parse::<GitInfo>()?;
        assert_eq!(git_info.branch_name, branch_name);
        Ok(())
    }

    #[test]
    fn uses_commit_sha_when_detached_head() -> anyhow::Result<()> {
        let status_output = "
            # branch.oid faeddf84c5077e7df0025334801d379bb94fc64f
            # branch.head (detached)
        "
        .trim();
        let git_info = status_output.parse::<GitInfo>()?;
        assert_eq!(git_info.branch_name, "(faeddf8...)");
        Ok(())
    }

    #[test]
    fn includes_stashes() -> anyhow::Result<()> {
        let status_output = "
            # branch.oid unused-invalid-sha
            # branch.head feature/spider-monkey
            # stash number-not-used
        "
        .trim();
        let git_info = status_output.parse::<GitInfo>()?;
        assert_eq!(git_info.stash, true);
        Ok(())
    }

    #[test]
    fn includes_untracked() -> anyhow::Result<()> {
        let status_output = "
            # branch.oid unused-invalid-sha
            # branch.head feature/hippopotamus
            ? path-not-used
        "
        .trim();
        let git_info = status_output.parse::<GitInfo>()?;
        assert_eq!(git_info.untracked, true);
        Ok(())
    }

    #[test]
    fn includes_staged() -> anyhow::Result<()> {
        let status_output = "
            # branch.oid unused-invalid-sha
            # branch.head feature/gorgonopsia
            1 Dwhatever
        "
        .trim();
        let git_info = status_output.parse::<GitInfo>()?;
        assert_eq!(git_info.staged, true);
        Ok(())
    }

    #[test]
    fn includes_unstaged() -> anyhow::Result<()> {
        let status_output = "
            # branch.oid unused-invalid-sha
            # branch.head feature/microraptoria
            1 WD
        "
        .trim();
        let git_info = status_output.parse::<GitInfo>()?;
        assert_eq!(git_info.unstaged, true);
        Ok(())
    }

    #[test]
    fn includes_branch_staged_untracked_incoming_remote() -> anyhow::Result<()> {
        let status_output = "
            # branch.oid unused-invalid-sha
            # branch.head feature/monothremes
            # branch.ab +0 -12
            2 D.
            1 M.
            ? not-used-file-name
        "
        .trim();
        let git_info = status_output.parse::<GitInfo>()?;
        assert_eq!(
            git_info,
            GitInfo {
                branch_name: "feature/monothremes".to_string(),
                staged: true,
                untracked: true,
                remote_diff: Some(GitRemoteDiff {
                    incoming: true,
                    outgoing: false
                }),
                stash: false,
                unstaged: false
            }
        );
        Ok(())
    }

    #[test]
    fn includes_sha_stash_unstaged_on_par_remote() -> anyhow::Result<()> {
        let status_output = "
            # branch.oid dfcac0b563036735405591415163566f9f908e1e
            # branch.head (detached)
            # branch.ab +0 -0
            # stash number-not-used
            u .D
            2 .R
        "
        .trim();
        let git_info = status_output.parse::<GitInfo>()?;
        assert_eq!(
            git_info,
            GitInfo {
                branch_name: "(dfcac0b...)".to_string(),
                stash: true,
                unstaged: true,
                remote_diff: Some(GitRemoteDiff {
                    incoming: false,
                    outgoing: false
                }),
                staged: false,
                untracked: false,
            }
        );
        Ok(())
    }

    #[test]
    fn includes_branch_staged_unstaged_no_remote() -> anyhow::Result<()> {
        let status_output = "
            # branch.oid unused-invalid-sha
            # branch.head feature/hymenoptera
            u .D
            2 D.
            1 DD
        "
        .trim();
        let git_info = status_output.parse::<GitInfo>()?;
        assert_eq!(
            git_info,
            GitInfo {
                branch_name: "feature/hymenoptera".to_string(),
                staged: true,
                unstaged: true,
                remote_diff: None,
                stash: false,
                untracked: false,
            }
        );
        Ok(())
    }

    mod git_remote_diff {
        use super::*;

        #[test]
        fn includes_remote_on_par() -> anyhow::Result<()> {
            let status_output = "# branch.ab +0 -0";
            let git_remote_diff = GitRemoteDiff::parse(status_output.lines())?;
            assert_eq!(
                git_remote_diff,
                Some(GitRemoteDiff {
                    incoming: false,
                    outgoing: false
                })
            );
            Ok(())
        }

        #[test]
        fn includes_remote_with_outgoing() -> anyhow::Result<()> {
            let status_output = "# branch.ab +7 -0";
            let git_remote_diff = GitRemoteDiff::parse(status_output.lines())?;
            assert_eq!(
                git_remote_diff,
                Some(GitRemoteDiff {
                    outgoing: true,
                    incoming: false,
                })
            );
            Ok(())
        }

        #[test]
        fn includes_remote_with_incoming() -> anyhow::Result<()> {
            let status_output = "# branch.ab +0 -3";
            let git_remote_diff = GitRemoteDiff::parse(status_output.lines())?;
            assert_eq!(
                git_remote_diff,
                Some(GitRemoteDiff {
                    outgoing: false,
                    incoming: true,
                })
            );
            Ok(())
        }

        #[test]
        fn includes_diverged_remote() -> anyhow::Result<()> {
            let status_output = "# branch.ab +9 -3";
            let git_remote_diff = GitRemoteDiff::parse(status_output.lines())?;
            assert_eq!(
                git_remote_diff,
                Some(GitRemoteDiff {
                    outgoing: true,
                    incoming: true,
                })
            );
            Ok(())
        }

        #[test]
        fn none_when_no_remote_line() -> anyhow::Result<()> {
            let status_output = "u AD";
            let git_remote_diff = GitRemoteDiff::parse(status_output.lines())?;
            assert!(git_remote_diff.is_none());
            Ok(())
        }

        #[test]
        fn finds_remote_line_amidst_other_output() -> anyhow::Result<()> {
            let status_lines = "
                # stash number-not-used
                # branch.ab +0 -0
                u .D
            "
            .trim()
            .lines()
            .map(str::trim);
            let git_remote_diff = GitRemoteDiff::parse(status_lines)?;
            assert!(git_remote_diff.is_some());
            Ok(())
        }

        #[test]
        fn displays_remote_on_par() {
            let git_remote_diff = GitRemoteDiff {
                incoming: false,
                outgoing: false,
            };
            assert_eq!(git_remote_diff.to_string(), "=");
        }

        #[test]
        fn displays_remote_outgoing() {
            let git_remote_diff = GitRemoteDiff {
                incoming: false,
                outgoing: true,
            };
            assert_eq!(git_remote_diff.to_string(), ">");
        }

        #[test]
        fn displays_remote_incoming() {
            let git_remote_diff = GitRemoteDiff {
                incoming: true,
                outgoing: false,
            };
            assert_eq!(git_remote_diff.to_string(), "<");
        }

        #[test]
        fn displays_remote_diverged() {
            let git_remote_diff = GitRemoteDiff {
                incoming: true,
                outgoing: true,
            };
            assert_eq!(git_remote_diff.to_string(), "<>");
        }
    }
}
