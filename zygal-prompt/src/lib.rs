mod git_info;
pub mod prompt;

#[derive(Debug, PartialEq)]
pub enum ZygalError {
    CurrentDirError,
    GitSpawnError,
    GitOutputError,
}
