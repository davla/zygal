mod git_info;
mod git_patch;
mod prompt;

include!(env!("CONFIG_IN"));

pub use prompt::prompt;
