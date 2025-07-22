mod git_info;
mod git_patch;
pub mod prompt;

#[derive(Debug, PartialEq)]
pub enum ZygalError {
    CurrentDirError,
    FromatError,
    GitSpawnError,
    GitOutputError,
}
