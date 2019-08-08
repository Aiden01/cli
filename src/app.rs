use crate::error::AppError;
use futures::compat::Future01CompatExt;
use reqwest::r#async::{
    multipart::{Form, Part},
    Client,
};
use serde_json::Value;
use std::fs::read_to_string;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "bayfiles-cli", about = "A CLI for Bayfiles")]
pub struct Context {
    /// File to upload
    #[structopt(name = "FILE", parse(from_os_str))]
    file: PathBuf,
    /// Whether to copy the link to the clipboard
    #[structopt(short = "c", long = "copy")]
    copy: bool,
}

pub async fn upload(context: Context) -> Result<String, AppError> {
    let Context { file, .. } = context;
    let client = Client::new();
    let buffer = read_to_string(&file)?;
    let filename = String::from(file.to_str().unwrap());
    let form = Form::new().part("file", Part::text(buffer).file_name(filename));
    let mut response = client
        .post("https://bayfiles.com/api/upload")
        .multipart(form)
        .send()
        .compat()
        .await?;
    let body: Value = response.json().compat().await?;
    let url = body.get("data")?.get("file")?.get("url")?.get("short")?;
    Ok(url.to_string())
}
