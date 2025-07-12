use regex::Regex;
use zygal::ZygalError;

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

    Ok(format!(
        "{branch_name} {unstaged}{staged}{stash}{untracked}{remote}"
    ))
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
