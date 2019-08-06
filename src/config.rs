use crate::error::BunnyError;
use serde_derive::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::{Path, PathBuf};
use xdg::BaseDirectories;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub token: String,
    pub connected: bool,
}

fn get_config_dirs() -> Result<BaseDirectories, BunnyError> {
    BaseDirectories::with_prefix("bunnycli").map_err(BunnyError::from)
}

fn get_config_file(path: PathBuf) -> Result<File, BunnyError> {
    let xdg_dirs = get_config_dirs()?;
    let config_path = match xdg_dirs.find_config_file(&path) {
        Some(x) => Ok(x),
        None => xdg_dirs.place_config_file(path),
    }?;

    File::open(config_path).map_err(BunnyError::from)
}

pub fn get_config(path: &Path) -> Result<Config, BunnyError> {
    let file = get_config_file(path.to_path_buf())?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    let config = toml::from_str(&contents)?;
    Ok(config)
}

pub fn write_config(path: &Path, config: Config) -> Result<(), BunnyError> {
    let toml = toml::to_string(&config)?;
    let mut file = get_config_file(path.to_path_buf())?;
    file.write_all(toml.as_bytes())?;
    Ok(())
}
