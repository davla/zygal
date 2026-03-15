use std::{io, result};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("File '{file_name}' not found at: {}", candidates.join(", "))]
    TomlNotFound {
        file_name: String,
        candidates: Vec<String>,
    },

    #[error("Failed to parse toml")]
    TomlParse(#[from] toml::de::Error),

    #[error("Failed to write to config.rs")]
    ConfigRsWrite(#[from] io::Error),
}

pub trait ErrExt<T, EIn, EOut>
where
    EOut: From<EIn>,
{
    fn err_into(self) -> result::Result<T, EOut>;
}

impl<T, EIn, EOut> ErrExt<T, EIn, EOut> for result::Result<T, EIn>
where
    EOut: From<EIn>,
{
    fn err_into(self) -> result::Result<T, EOut> {
        self.map_err(|err| err.into())
    }
}
