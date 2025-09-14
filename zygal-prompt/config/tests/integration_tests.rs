use std::io::{Read, Write};

use asserting::prelude::*;
use tempfile::{NamedTempFile, TempDir};

use zygal_config::write_config;

#[test]
fn config_toml_and_color_scheme_are_used() {
    let tmp_dir = temp_dir();
    let config_input = config_toml(&tmp_dir, true, "something-short");
    let color_scheme = color_scheme_toml(&tmp_dir, [(81, 29), (192, 0), (219, 63)]);
    let mut config_output_path = create_temp_file(&tmp_dir);

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
    pub const SHELL: &str = "zsh";
    pub const RESET_STYLE: &str = "%f%k";

    pub const CURRENT_DIR_PREFIX: &str = "%F{29}%K{81} ";
    pub const CURRENT_DIR_SUFFIX: &str = " ";

    pub const GIT_PREFIX: &str = "%F{0}%K{192} ";
    pub const GIT_SUFFIX: &str = " ";

    pub const NEW_LINE: &str = "%F{63}%K{219} something-short ";
}
"#,
    );
}

#[test]
fn no_padding_if_space_around_is_false() {
    let tmp_dir = temp_dir();
    let config_input = config_toml(&tmp_dir, false, "%#");
    let color_scheme = color_scheme_toml(&tmp_dir, [(0, 0), (0, 0), (0, 0)]);
    let mut config_output_path = create_temp_file(&tmp_dir);

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
    pub const SHELL: &str = "zsh";
    pub const RESET_STYLE: &str = "%f%k";

    pub const CURRENT_DIR_PREFIX: &str = "%F{0}%K{0}";
    pub const CURRENT_DIR_SUFFIX: &str = "";

    pub const GIT_PREFIX: &str = "%F{0}%K{0}";
    pub const GIT_SUFFIX: &str = "";

    pub const NEW_LINE: &str = "%F{0}%K{0}%#";
}
"#,
    );
}

#[test]
fn no_color_in_color_scheme_is_reset() {
    let tmp_dir = temp_dir();
    let config_input = config_toml(&tmp_dir, false, "%#");

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
    assert_that(config_output).is_equal_to(
        r#"mod config {
    pub const SHELL: &str = "zsh";
    pub const RESET_STYLE: &str = "%f%k";

    pub const CURRENT_DIR_PREFIX: &str = "%F{22}%k";
    pub const CURRENT_DIR_SUFFIX: &str = "";

    pub const GIT_PREFIX: &str = "%f%K{7}";
    pub const GIT_SUFFIX: &str = "";

    pub const NEW_LINE: &str = "%f%k%#";
}
"#,
    );
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

fn config_toml(tmp_dir: &TempDir, space_around: bool, new_line_content: &str) -> NamedTempFile {
    write_temp_file(
        tmp_dir,
        &format!(
            r#"
shell = "zsh"
new-line-content = "{new_line_content}"
space-around = {space_around}
    "#,
        ),
    )
}

fn color_scheme_toml(tmp_dir: &TempDir, colors: [(u8, u8); 3]) -> NamedTempFile {
    write_temp_file(
        tmp_dir,
        &format!(
            r#"[current-dir]
background = {}
foreground = {}

[git]
background = {}
foreground = {}

[new-line]
background = {}
foreground = {}
"#,
            colors[0].0, colors[0].1, colors[1].0, colors[1].1, colors[2].0, colors[2].1,
        ),
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
