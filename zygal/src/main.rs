use std::process;

use regex::Regex;

const PRE_GIT: &str = "%F{0}%K{208} %n@%M %K{220} %3(~.*/%1~.%~) %f%k";
const POST_GIT: &str = "\n%F{0}%K{208} %# %f%k ";

fn main() -> Result<(), ZygalError> {
    let git_status_output = {
        let child_output = process::Command::new("git")
            .args(["status", "--porcelain=v2", "--branch", "--show-stash"])
            .output()
            .map_err(|_| ZygalError::GitSpawnError)?;
        if !child_output.status.success() {
            return Ok(());
        }
        String::from_utf8(child_output.stdout).map_err(|_| ZygalError::GitOutputError)?
    };

    let git_info = make_git_info(&git_status_output)?;
    println!("{PRE_GIT} {git_info} {POST_GIT}");
    Ok(())
}

#[derive(Debug)]
enum ZygalError {
    GitSpawnError,
    GitOutputError,
}

fn make_git_info(git_status_output: &str) -> Result<String, ZygalError> {
    let mut lines = git_status_output.lines().map(str::trim);

    let branch_name = make_branch_name(&mut lines)?;

    let lines: Vec<&str> = lines.collect();
    let remote = make_remote_symbols(lines.iter())?;
    let stash = any_or_empty(lines.iter(), |l| l.starts_with("# stash"), "$");
    let untracked = any_or_empty(lines.iter(), |l| l.starts_with("?"), "%");

    let staged_regex = Regex::new(r"^[12u] [MTARCDU]\.").unwrap();
    let staged = any_or_empty(lines.iter(), |l| staged_regex.is_match(l), "+");

    let unstaged_regex = Regex::new(r"^[12u] \.[MTARCDU]").unwrap();
    let unstaged = any_or_empty(lines.iter(), |l| unstaged_regex.is_match(l), "*");

    Ok(format!(
        "{branch_name} {unstaged}{staged}{stash}{untracked}{remote}"
    ))
}

fn make_branch_name<'a>(
    git_status_lines: &mut impl Iterator<Item = &'a str>,
) -> Result<String, ZygalError> {
    let sha = git_status_lines
        .next()
        .ok_or(ZygalError::GitOutputError)?
        .split_whitespace()
        .nth(2)
        .ok_or(ZygalError::GitOutputError)?;

    let branch_name = git_status_lines
        .next()
        .ok_or(ZygalError::GitOutputError)?
        .split_whitespace()
        .nth(2)
        .ok_or(ZygalError::GitOutputError)?;

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

    let ahead_count = counts
        .next()
        .ok_or(ZygalError::GitOutputError)?
        .parse::<i64>()
        .map_err(|_| ZygalError::GitOutputError)?;
    let behind_count = counts
        .next()
        .ok_or(ZygalError::GitOutputError)?
        .parse::<i64>()
        .map_err(|_| ZygalError::GitOutputError)?;

    Ok(match (ahead_count, behind_count) {
        (0, 0) => "=",
        (0, _) => ">",
        (_, 0) => "<",
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
