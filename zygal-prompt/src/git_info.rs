use regex::Regex;

use crate::ZygalError;

pub fn make_git_info(git_status_output: &str) -> Result<String, ZygalError> {
    let mut lines = git_status_output.lines().map(str::trim);

    let branch_name = make_branch_name(&mut lines)?;

    let lines: Vec<&str> = lines.collect();
    let remote = make_remote_symbols(lines.iter())?;
    let stash = any_or_empty(lines.iter(), |l| l.starts_with("# stash"), "$");
    let untracked = any_or_empty(lines.iter(), |l| l.starts_with("?"), "%");

    let staged_regex = Regex::new(r"^[12u] [MTARCDU].").unwrap();
    let staged = any_or_empty(lines.iter(), |l| staged_regex.is_match(l), "+");

    let unstaged_regex = Regex::new(r"^[12u] .[MTARCDU]").unwrap();
    let unstaged = any_or_empty(lines.iter(), |l| unstaged_regex.is_match(l), "*");

    let git_info = format!("{branch_name} {unstaged}{staged}{stash}{untracked}{remote}");
    Ok(git_info.trim().to_string())
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

fn make_remote_symbols<'a>(
    mut git_status_lines: impl Iterator<Item = &'a &'a str>,
) -> Result<&'static str, ZygalError> {
    let Some(ab_line) = git_status_lines.find(|l| l.starts_with("# branch.ab")) else {
        return Ok("");
    };

    let mut counts = ab_line.split_whitespace().skip(2);

    let incoming_count = counts
        .advance()?
        .parse::<i64>()
        .map_err(|_| ZygalError::GitOutputError)?;
    let outgoing_count = counts
        .advance()?
        .parse::<i64>()
        .map_err(|_| ZygalError::GitOutputError)?;

    Ok(match (incoming_count, outgoing_count) {
        (0, 0) => "=",
        (0, _) => "<",
        (_, 0) => ">",
        (_, _) => "<>",
    })
}

fn any_or_empty<'a, F>(
    mut lines: impl Iterator<Item = &'a &'a str>,
    f: F,
    symbol: &'static str,
) -> &'static str
where
    F: Fn(&'a &'a str) -> bool,
{
    if lines.any(f) { symbol } else { "" }
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
        let git_info = make_git_info(status_output);
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
        let git_info = make_git_info(status_output.trim());
        assert_eq!(git_info.as_deref(), Ok(branch_name));
    }

    #[test]
    fn uses_commit_sha_when_detached_head() {
        let status_output = "
            # branch.oid faeddf84c5077e7df0025334801d379bb94fc64f
            # branch.head (detached)
        "
        .trim();
        let git_info = make_git_info(status_output);
        assert_eq!(git_info.as_deref(), Ok("(faeddf8...)"));
    }

    #[test]
    fn includes_stashes() {
        let status_output = "
            # branch.oid unused-invalid-sha
            # branch.head feature/spider-monkey
            # stash number-not-used
        "
        .trim();
        let git_info = make_git_info(status_output);
        assert!(git_info.is_ok_and(|info| info.contains("$")))
    }

    #[test]
    fn includes_untracked() {
        let status_output = "
            # branch.oid unused-invalid-sha
            # branch.head feature/hippopotamus
            ? path-not-used
        "
        .trim();
        let git_info = make_git_info(status_output);
        assert!(git_info.is_ok_and(|info| info.contains("%")))
    }

    #[test]
    fn includes_staged() {
        let status_output = "
            # branch.oid unused-invalid-sha
            # branch.head feature/gorgonopsia
            1 Dwhatever
        "
        .trim();
        let git_info = make_git_info(status_output);
        assert!(git_info.is_ok_and(|info| info.contains("+")))
    }

    #[test]
    fn includes_unstaged() {
        let status_output = "
            # branch.oid unused-invalid-sha
            # branch.head feature/microraptoria
            1 WD
        "
        .trim();
        let git_info = make_git_info(status_output);
        assert!(git_info.is_ok_and(|info| info.contains("*")))
    }

    #[test]
    fn includes_remote_on_par() {
        let status_output = "
            # branch.oid unused-invalid-sha
            # branch.head feature/placoderms
            # branch.ab +0 -0
        "
        .trim();
        let git_info = make_git_info(status_output);
        assert!(git_info.is_ok_and(|info| info.contains("=")))
    }

    #[test]
    fn includes_remote_with_outgoing() {
        let status_output = "
            # branch.oid unused-invalid-sha
            # branch.head feature/ichthyosauria
            # branch.ab +7 -0
        "
        .trim();
        let git_info = make_git_info(status_output);
        assert!(git_info.is_ok_and(|info| info.contains(">")))
    }

    #[test]
    fn includes_remote_with_incoming() {
        let status_output = "
            # branch.oid unused-invalid-sha
            # branch.head feature/hyaenodonta
            # branch.ab +0 -3
        "
        .trim();
        let git_info = make_git_info(status_output);
        assert!(git_info.is_ok_and(|info| info.contains("<")))
    }

    #[test]
    fn includes_diverged_remote() {
        let status_output = "
            # branch.oid unused-invalid-sha
            # branch.head feature/ceratopsia
            # branch.ab +9 -3
        "
        .trim();
        let git_info = make_git_info(status_output);
        assert!(git_info.is_ok_and(|info| info.contains("<>")))
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
        let git_info = make_git_info(status_output);
        assert_eq!(git_info.as_deref(), Ok("feature/monothremes +%<"))
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
        let git_info = make_git_info(status_output);
        assert_eq!(git_info.as_deref(), Ok("(dfcac0b...) *$="))
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
        let git_info = make_git_info(status_output);
        assert_eq!(git_info.as_deref(), Ok("feature/hymenoptera *+"))
    }
}
