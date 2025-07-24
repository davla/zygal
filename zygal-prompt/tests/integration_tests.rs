use std::{fs, path::Path, process};

use anyhow::Context;
use tempdir::TempDir;
use zygal_prompt::prompt::prompt;

#[test]
fn no_git_info_when_not_in_git_repository() -> anyhow::Result<()> {
    let tmp_dir = mktemp()?;
    assert_eq!(
        prompt(tmp_dir.path())?,
        "%F{0}%K{208} %3(~.*/%1~.%~) %f%k\n%F{0}%K{208} %# %f%k "
    );
    Ok(())
}

#[test]
fn includes_git_info_when_in_git_repository() -> anyhow::Result<()> {
    let tmp_dir = mktemp()?;
    let repo_root = tmp_dir.path();

    let branch = "archaea";
    git_init(branch, repo_root)?;

    assert_eq!(
        prompt(repo_root)?,
        format!(
            "%F{{0}}%K{{208}} %3(~.*/%1~.%~) %K{{220}} [{branch}] %f%k\n%F{{0}}%K{{208}} %# %f%k "
        )
    );
    Ok(())
}

#[test]
fn includes_merging_when_merge_conflicts() -> anyhow::Result<()> {
    let tmp_dir = mktemp()?;
    let repo_root = tmp_dir.path();

    let (main_branch, other_branch) = ("angiosperms", "gymnosperms");
    git_init(main_branch, repo_root)?;
    create_conflicting_files(repo_root, main_branch, other_branch)?;
    spawn_git(&["merge", other_branch], repo_root, true)?;

    assert_eq!(
        prompt(repo_root)?,
        format!(
            "%F{{0}}%K{{208}} %3(~.*/%1~.%~) %K{{220}} [{main_branch} M*+] %f%k\n%F{{0}}%K{{208}} %# %f%k "
        )
    );
    Ok(())
}

#[test]
fn includes_rebasing_when_rebase_conflicts() -> anyhow::Result<()> {
    let tmp_dir = mktemp()?;
    let repo_root = tmp_dir.path();

    let (main_branch, other_branch) = ("cephalopods", "octopoda");
    git_init(main_branch, repo_root)?;
    create_conflicting_files(repo_root, main_branch, other_branch)?;
    git(&["switch", other_branch], repo_root)?;
    spawn_git(
        &["rebase", "--onto", main_branch, "HEAD^", other_branch],
        repo_root,
        true,
    )?;

    let sha = git(&["rev-parse", "--short", "HEAD"], repo_root)?;
    assert_eq!(
        prompt(repo_root)?,
        format!(
            "%F{{0}}%K{{208}} %3(~.*/%1~.%~) %K{{220}} [({}...) B*+] %f%k\n%F{{0}}%K{{208}} %# %f%k ",
            sha.trim()
        )
    );
    Ok(())
}

#[test]
fn includes_cherry_pick_when_cherry_pick_conflicts() -> anyhow::Result<()> {
    let tmp_dir = mktemp()?;
    let repo_root = tmp_dir.path();

    let (main_branch, other_branch) = ("chondrichthyes", "osteichthyes");
    git_init(main_branch, repo_root)?;
    create_conflicting_files(repo_root, main_branch, other_branch)?;
    spawn_git(&["cherry-pick", other_branch], repo_root, true)?;

    assert_eq!(
        prompt(repo_root)?,
        format!(
            "%F{{0}}%K{{208}} %3(~.*/%1~.%~) %K{{220}} [{main_branch} H*+] %f%k\n%F{{0}}%K{{208}} %# %f%k "
        )
    );
    Ok(())
}

#[test]
fn includes_revert_when_revert_conflicts() -> anyhow::Result<()> {
    let tmp_dir = mktemp()?;
    let repo_root = tmp_dir.path();

    let branch = "arthropoda";
    git_init(branch, repo_root)?;

    let file_path = repo_root.join("limbs.txt");
    fs::write(&file_path, "The more the merrier")
        .context(format!("Failed the first write to file {file_path:?}"))?;
    git(&["add", "--all"], repo_root)?;
    git(&["commit", "--message", "Add myriapoda"], repo_root)?;
    fs::write(&file_path, "6 is good enough")
        .context(format!("Failed the second write to file {file_path:?}"))?;
    git(&["commit", "--all", "--message", "Add hexapoda"], repo_root)?;
    spawn_git(&["revert", "HEAD^"], repo_root, true)?;

    assert_eq!(
        prompt(repo_root)?,
        format!(
            "%F{{0}}%K{{208}} %3(~.*/%1~.%~) %K{{220}} [{branch} V*+] %f%k\n%F{{0}}%K{{208}} %# %f%k "
        )
    );
    Ok(())
}

fn mktemp() -> anyhow::Result<TempDir> {
    TempDir::new("zygal-prompt-test")
        .context("Failed to create temporary directory in integration tests")
}

fn git(args: &[&str], current_dir: &Path) -> anyhow::Result<String> {
    spawn_git(args, current_dir, false)
}

fn spawn_git(args: &[&str], current_dir: &Path, expect_failure: bool) -> anyhow::Result<String> {
    let stdout = process::Command::new("git")
        .args(args)
        .current_dir(current_dir)
        .output()
        .ok()
        .filter(|output| {
            let success = output.status.success();
            if expect_failure { !success } else { success }
        })
        .context(format!("Failed to run 'git {args:?}' in integration tests"))?
        .stdout;
    String::from_utf8(stdout).context(format!(
        "Failed to parse git {args:?} output in integration tests"
    ))
}

fn git_init(branch: &str, current_dir: &Path) -> anyhow::Result<()> {
    git(&["init", "--initial-branch", branch], current_dir)?;
    git(
        &["commit", "--allow-empty", "--message", "Cambrian explosion"],
        current_dir,
    )?;
    Ok(())
}

fn create_conflicting_files(
    current_dir: &Path,
    current_branch: &str,
    other_branch: &str,
) -> anyhow::Result<()> {
    let file_path = current_dir.join("amniotes.txt");

    git(&["switch", current_branch], current_dir)?;
    fs::write(&file_path, "sauropsida").context(format!(
        "Failed to write to file {file_path:?} on branch {current_branch}"
    ))?;
    git(&["add", "--all"], current_dir)?;
    git(
        &["commit", "--message", "Use lighter skeleton"],
        current_dir,
    )?;

    git(&["switch", "--create", other_branch], current_dir)?;
    git(&["reset", "--keep", "HEAD^"], current_dir)?;
    fs::write(&file_path, "synapsida").context(format!(
        "Failed to write to file {file_path:?} on branch {other_branch}"
    ))?;
    git(&["add", "--all"], current_dir)?;
    git(
        &["commit", "--message", "Add teeth differenciation"],
        current_dir,
    )?;

    git(&["switch", current_branch], current_dir)?;
    Ok(())
}
