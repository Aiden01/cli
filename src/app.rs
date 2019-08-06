use std::path::{Path, PathBuf};
use structopt::StructOpt;
use promptly::prompt;

use crate::config::{get_config, write_config, Config};
use crate::error::BunnyError;

#[derive(Debug, StructOpt)]
#[structopt(name = "bunnycli", about = "A CLI for BunnyCdn")]
pub enum Command {
    #[structopt(name = "login")]
    Login,
    /// Uploads a file
    #[structopt(name = "upload")]
    Upload {
        /// File to upload
        #[structopt(name = "FILE", parse(from_os_str))]
        file: PathBuf,
        /// Whether to copy the link to the clipboard
        #[structopt(short = "c", long = "copy")]
        copy: bool,
    },
}

fn login() -> Result<(), BunnyError> {
    let token: String = prompt("Enter your token: ");
    let config = Config { token, connected: true };
    write_config(Path::new("config.toml"), config)?;
    println!("You are successfully connected.");
    Ok(())
}

fn upload(file: PathBuf, copy: bool, config: Config) -> Result<(), BunnyError> {
    let Config { token, connected } = config;
    if !connected {
        println!("You are not connected. Type bunnycli login to connect.");
        return Ok(());
    }

    Ok(())
}

pub fn run(command: Command) -> Result<(), BunnyError> {
    let config = get_config(Path::new("config.toml"))?;
    match command {
        Command::Login => login(),
        Command::Upload { file, copy } => upload(file, copy, config),
    }
}
