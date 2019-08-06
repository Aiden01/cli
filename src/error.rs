use std::convert::From;
use std::io::Error;
use toml::{de, ser};
use xdg::BaseDirectoriesError;

pub enum BunnyError {
    ConfigError(String),
    IOError(String),
    ParseError(String),
}

impl From<de::Error> for BunnyError {
    fn from(error: de::Error) -> Self {
        BunnyError::ParseError(error.to_string())
    }
}

impl From<ser::Error> for BunnyError {
    fn from(error: ser::Error) -> Self {
        BunnyError::ParseError(error.to_string())
    }
}

impl From<Error> for BunnyError {
    fn from(error: Error) -> Self {
        BunnyError::IOError(error.to_string())
    }
}

impl From<BaseDirectoriesError> for BunnyError {
    fn from(error: BaseDirectoriesError) -> Self {
        BunnyError::ConfigError(error.to_string())
    }
}
