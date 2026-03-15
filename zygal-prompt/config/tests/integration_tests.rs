use std::io::{Read, Write};

use asserting::prelude::*;
use tempfile::{NamedTempFile, TempDir};

use zygal_config::write_config;

#[test]
fn config_toml_and_color_scheme_are_used() {
    let tmp_dir = temp_dir();
    let mut config_output_path = create_temp_file(&tmp_dir);

    let config_input = write_temp_file(
        &tmp_dir,
        r#"
shell = "zsh"
new-line-content = "something-short"
space-around = true

[git]
merge = "@"
rebase = "_"
cherry-pick = "|"
revert = ":"
unstaged = "^"
staged = "&"
stash = "!"
untracked = "??"

[git.remote]
ahead = "++"
behind = "--"
on-par = "~~"
"#,
    );

    let color_scheme = write_temp_file(
        &tmp_dir,
        "
[current-dir]
background = 81
foreground = 29

[git]
background = 192
foreground = 0

[new-line]
background = 219
foreground = 63
",
    );

    let write_config_result = write_config(
        config_output_path.path(),
        &config_input.to_string(),
        &color_scheme.to_string(),
    );
    assert_that(write_config_result).is_ok();

    let mut config_output = String::new();
    assert_that(config_output_path.read_to_string(&mut config_output)).is_ok();
    assert_that(config_output).is_equal_to(
        r#"mod config {
    pub struct GitRemote {
        pub ahead: &'static str,
        pub behind: &'static str,
        pub on_par: &'static str,
    }

    pub const SHELL: &str = "zsh";
    pub const RESET_STYLE: &str = "%f%k";

    pub const CURRENT_DIR_PREFIX: &str = "%F{29}%K{81} ";
    pub const CURRENT_DIR_SUFFIX: &str = " ";

    pub const GIT_PREFIX: &str = "%F{0}%K{192} ";
    pub const GIT_SUFFIX: &str = " ";

    pub const NEW_LINE: &str = "%F{63}%K{219} something-short ";

    pub const GIT_MERGE: Option<&str> = Some("@");
    pub const GIT_REBASE: Option<&str> = Some("_");
    pub const GIT_CHERRY_PICK: Option<&str> = Some("|");
    pub const GIT_REVERT: Option<&str> = Some(":");

    pub const GIT_UNSTAGED: Option<&str> = Some("^");
    pub const GIT_STAGED: Option<&str> = Some("&");
    pub const GIT_STASH: Option<&str> = Some("!");
    pub const GIT_UNTRACKED: Option<&str> = Some("??");

    pub const GIT_REMOTE: Option<GitRemote> = Some(GitRemote { ahead: "++", behind: "--", on_par: "~~" });
}
"#,
    );
}

#[test]
fn empty_string_in_git_symbols_is_none() {
    let tmp_dir = temp_dir();
    let color_scheme = color_scheme_toml(&tmp_dir);
    let mut config_output_path = create_temp_file(&tmp_dir);

    let config_input = write_temp_file(
        &tmp_dir,
        r#"
shell = "zsh"
new-line-content = "%#"
space-around = true

[git]
merge = ""
rebase = ""
cherry-pick = ""
revert = ""
unstaged = ""
staged = ""
stash = ""
untracked = ""
"#,
    );

    let write_config_result = write_config(
        config_output_path.path(),
        &config_input.to_string(),
        &color_scheme.to_string(),
    );
    assert_that(write_config_result).is_ok();

    let mut config_output = String::new();
    assert_that(config_output_path.read_to_string(&mut config_output)).is_ok();
    assert_that(config_output).contains(
        r#"
    pub const GIT_MERGE: Option<&str> = None;
    pub const GIT_REBASE: Option<&str> = None;
    pub const GIT_CHERRY_PICK: Option<&str> = None;
    pub const GIT_REVERT: Option<&str> = None;

    pub const GIT_UNSTAGED: Option<&str> = None;
    pub const GIT_STAGED: Option<&str> = None;
    pub const GIT_STASH: Option<&str> = None;
    pub const GIT_UNTRACKED: Option<&str> = None;
"#,
    );
}

#[test]
fn no_padding_if_space_around_is_false() {
    let tmp_dir = temp_dir();
    let color_scheme = color_scheme_toml(&tmp_dir);
    let mut config_output_path = create_temp_file(&tmp_dir);

    let config_input = write_temp_file(
        &tmp_dir,
        r#"
shell = "zsh"
new-line-content = "%#"
space-around = false

[git]
"#,
    );

    let write_config_result = write_config(
        config_output_path.path(),
        &config_input.to_string(),
        &color_scheme.to_string(),
    );
    assert_that(write_config_result).is_ok();

    let mut config_output = String::new();
    assert_that(config_output_path.read_to_string(&mut config_output)).is_ok();
    assert_that(config_output).contains(
        r#"
    pub const CURRENT_DIR_PREFIX: &str = "%F{0}%K{0}";
    pub const CURRENT_DIR_SUFFIX: &str = "";

    pub const GIT_PREFIX: &str = "%F{0}%K{0}";
    pub const GIT_SUFFIX: &str = "";

    pub const NEW_LINE: &str = "%F{0}%K{0}%#";
"#,
    );
}

#[test]
fn no_git_symbol_is_none() {
    let tmp_dir = temp_dir();
    let color_scheme = color_scheme_toml(&tmp_dir);

    let config_input = write_temp_file(
        &tmp_dir,
        r#"
shell = "zsh"
new-line-content = "%#"
space-around = true

[git]
"#,
    );

    let mut config_output_path = create_temp_file(&tmp_dir);

    let write_config_result = write_config(
        config_output_path.path(),
        &config_input.to_string(),
        &color_scheme.to_string(),
    );
    assert_that(write_config_result).is_ok();

    let mut config_output = String::new();
    assert_that(config_output_path.read_to_string(&mut config_output)).is_ok();
    assert_that(config_output).contains(
        r#"
    pub const GIT_MERGE: Option<&str> = None;
    pub const GIT_REBASE: Option<&str> = None;
    pub const GIT_CHERRY_PICK: Option<&str> = None;
    pub const GIT_REVERT: Option<&str> = None;

    pub const GIT_UNSTAGED: Option<&str> = None;
    pub const GIT_STAGED: Option<&str> = None;
    pub const GIT_STASH: Option<&str> = None;
    pub const GIT_UNTRACKED: Option<&str> = None;

    pub const GIT_REMOTE: Option<GitRemote> = None;
"#,
    );
}

#[test]
fn no_color_is_reset() {
    let tmp_dir = temp_dir();
    let config_input = config_toml(&tmp_dir);

    let color_scheme = write_temp_file(
        &tmp_dir,
        r#"
[current-dir]
foreground = 22

[git]
background = 7

[new-line]
foreground = "reset"
"#,
    );

    let mut config_output_path = create_temp_file(&tmp_dir);

    let write_config_result = write_config(
        config_output_path.path(),
        &config_input.to_string(),
        &color_scheme.to_string(),
    );
    assert_that(write_config_result).is_ok();

    let mut config_output = String::new();
    assert_that(config_output_path.read_to_string(&mut config_output)).is_ok();
    assert_that(&config_output).contains(r#"pub const CURRENT_DIR_PREFIX: &str = "%F{22}%k ";"#);
    assert_that(&config_output).contains(r#"pub const GIT_PREFIX: &str = "%f%K{7} ";"#);
    assert_that(&config_output).contains(r#"pub const NEW_LINE: &str = "%f%k %# ";"#);
}

fn temp_dir() -> TempDir {
    TempDir::with_prefix("zygal-config-test")
        .expect("Failed to create temporary directory in config integration tests")
}

fn create_temp_file(tmp_dir: &TempDir) -> NamedTempFile {
    let err_msg = format!(
        "Failed to create temporary file in directory {} in config integration tests",
        tmp_dir.path().display()
    );
    NamedTempFile::new_in(tmp_dir).expect(&err_msg)
}

fn config_toml(tmp_dir: &TempDir) -> NamedTempFile {
    write_temp_file(
        tmp_dir,
        r#"
shell = "zsh"
new-line-content = "%#"
space-around = true

[git]
"#,
    )
}

fn color_scheme_toml(tmp_dir: &TempDir) -> NamedTempFile {
    write_temp_file(
        tmp_dir,
        "
[current-dir]
background = 0
foreground = 0

[git]
background = 0
foreground = 0

[new-line]
background = 0
foreground = 0
",
    )
}

fn write_temp_file(tmp_dir: &TempDir, content: &str) -> NamedTempFile {
    let mut file = create_temp_file(tmp_dir);
    let err_msg = format!(
        "Failed to write to temporary file {} in config integration tests",
        file.path().display()
    );
    write!(&mut file, "{}", content).expect(&err_msg);
    file
}

trait TempFileExt {
    fn to_string(&self) -> String;
}

impl TempFileExt for NamedTempFile {
    fn to_string(&self) -> String {
        self.path().display().to_string()
    }
}
