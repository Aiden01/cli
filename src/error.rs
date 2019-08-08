use reqwest::Error as ReqwestError;
use std::convert::From;
use std::io::Error;
use toml::{de, ser};
use xdg::BaseDirectoriesError;

#[derive(Debug)]
pub enum AppError {
    ConfigError(String),
    IOError(String),
    ParseError(String),
    HttpError(String),
    Other(String),
}

impl From<std::option::NoneError> for AppError {
    fn from(_: std::option::NoneError) -> Self {
        AppError::Other(String::from("None"))
    }
}

impl From<ReqwestError> for AppError {
    fn from(error: ReqwestError) -> Self {
        AppError::HttpError(error.to_string())
    }
}

impl From<de::Error> for AppError {
    fn from(error: de::Error) -> Self {
        AppError::ParseError(error.to_string())
    }
}

impl From<ser::Error> for AppError {
    fn from(error: ser::Error) -> Self {
        AppError::ParseError(error.to_string())
    }
}

impl From<Error> for AppError {
    fn from(error: Error) -> Self {
        AppError::IOError(error.to_string())
    }
}

impl From<BaseDirectoriesError> for AppError {
    fn from(error: BaseDirectoriesError) -> Self {
        AppError::ConfigError(error.to_string())
    }
}
