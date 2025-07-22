use std::{fs, path::Path, process};

use tempdir::TempDir;
use zygal_prompt::prompt::prompt;

#[test]
fn no_git_info_when_not_in_git_repository() {
    let tmp_dir = mktemp();
    assert_eq!(
        prompt(tmp_dir.path()).as_deref(),
        Ok("%F{0}%K{208} %3(~.*/%1~.%~) %f%k\n%F{0}%K{208} %# %f%k ")
    );
}

#[test]
fn includes_git_info_when_in_git_repository() {
    let tmp_dir = mktemp();
    let repo_root = tmp_dir.path();

    let branch = "archaea";
    git_init(branch, repo_root);

    assert_eq!(
        prompt(repo_root),
        Ok(format!(
            "%F{{0}}%K{{208}} %3(~.*/%1~.%~) %K{{220}} [{branch}] %f%k\n%F{{0}}%K{{208}} %# %f%k "
        ))
    );
}

#[test]
fn includes_merging_when_merge_conflicts() {
    let tmp_dir = mktemp();
    let repo_root = tmp_dir.path();

    let (main_branch, other_branch) = ("angiosperms", "gymnosperms");
    git_init(main_branch, repo_root);
    create_conflicting_files(repo_root, main_branch, other_branch);
    spawn_git(&["merge", other_branch], repo_root, true);

    assert_eq!(
        prompt(repo_root),
        Ok(format!(
            "%F{{0}}%K{{208}} %3(~.*/%1~.%~) %K{{220}} [{main_branch} M*+] %f%k\n%F{{0}}%K{{208}} %# %f%k "
        ))
    );
}

#[test]
fn includes_rebasing_when_rebase_conflicts() {
    let tmp_dir = mktemp();
    let repo_root = tmp_dir.path();

    let (main_branch, other_branch) = ("cephalopods", "octopoda");
    git_init(main_branch, repo_root);
    create_conflicting_files(repo_root, main_branch, other_branch);
    git(&["switch", other_branch], repo_root);
    spawn_git(
        &["rebase", "--onto", main_branch, "HEAD^", other_branch],
        repo_root,
        true,
    );

    let sha = git(&["rev-parse", "--short", "HEAD"], repo_root);
    assert_eq!(
        prompt(repo_root),
        Ok(format!(
            "%F{{0}}%K{{208}} %3(~.*/%1~.%~) %K{{220}} [({}...) B*+] %f%k\n%F{{0}}%K{{208}} %# %f%k ",
            sha.trim()
        ))
    );
}

#[test]
fn includes_cherry_pick_when_cherry_pick_conflicts() {
    let tmp_dir = mktemp();
    let repo_root = tmp_dir.path();

    let (main_branch, other_branch) = ("chondrichthyes", "osteichthyes");
    git_init(main_branch, repo_root);
    create_conflicting_files(repo_root, main_branch, other_branch);
    spawn_git(&["cherry-pick", other_branch], repo_root, true);

    assert_eq!(
        prompt(repo_root),
        Ok(format!(
            "%F{{0}}%K{{208}} %3(~.*/%1~.%~) %K{{220}} [{main_branch} H*+] %f%k\n%F{{0}}%K{{208}} %# %f%k "
        ))
    );
}

#[test]
fn includes_revert_when_revert_conflicts() {
    let tmp_dir = mktemp();
    let repo_root = tmp_dir.path();

    let branch = "arthropoda";
    git_init(branch, repo_root);

    let file_path = repo_root.join("limbs.txt");
    fs::write(&file_path, "The more the merrier").unwrap();
    git(&["add", "--all"], repo_root);
    git(&["commit", "--message", "Add myriapoda"], repo_root);
    fs::write(&file_path, "6 is good enough").unwrap();
    git(&["commit", "--all", "--message", "Add hexapoda"], repo_root);
    spawn_git(&["revert", "HEAD^"], repo_root, true);

    assert_eq!(
        prompt(repo_root),
        Ok(format!(
            "%F{{0}}%K{{208}} %3(~.*/%1~.%~) %K{{220}} [{branch} V*+] %f%k\n%F{{0}}%K{{208}} %# %f%k "
        ))
    );
}

fn mktemp() -> TempDir {
    TempDir::new("zygal-prompt-test")
        .expect("Failed to create temporary directory in integration tests")
}

fn git(args: &[&str], current_dir: &Path) -> String {
    spawn_git(args, current_dir, false)
}

fn spawn_git(args: &[&str], current_dir: &Path, expect_failure: bool) -> String {
    let cmd_err_msg = format!("Failed to run 'git {args:?}' in integration tests");
    let stdout = process::Command::new("git")
        .args(args)
        .current_dir(current_dir)
        .output()
        .ok()
        .filter(|output| {
            let success = output.status.success();
            if expect_failure { !success } else { success }
        })
        .expect(&cmd_err_msg)
        .stdout;
    let stdout_err_msg = format!("Failed to parse git {args:?} output in integration tests");
    String::from_utf8(stdout).expect(&stdout_err_msg)
}

fn git_init(branch: &str, current_dir: &Path) {
    git(&["init", "--initial-branch", branch], current_dir);
    git(
        &["commit", "--allow-empty", "--message", "Cambrian explosion"],
        current_dir,
    );
}

fn create_conflicting_files(current_dir: &Path, current_branch: &str, other_branch: &str) {
    let file_path = current_dir.join("amniotes.txt");

    git(&["switch", current_branch], current_dir);
    fs::write(&file_path, "sauropsida").unwrap();
    git(&["add", "--all"], current_dir);
    git(
        &["commit", "--message", "Use lighter skeleton"],
        current_dir,
    );

    git(&["switch", "--create", other_branch], current_dir);
    git(&["reset", "--keep", "HEAD^"], current_dir);
    fs::write(&file_path, "synapsida").unwrap();
    git(&["add", "--all"], current_dir);
    git(
        &["commit", "--all", "--message", "Add teeth differenciation"],
        current_dir,
    );

    git(&["switch", current_branch], current_dir);
}
