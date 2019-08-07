use futures::compat::Future01CompatExt;
use hyper::client::Client;
use hyper::header::CONTENT_TYPE;
use hyper::{Body, Method, Request};
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use structopt::StructOpt;
use crate::error::AppError;

const BOUNDARY: &'static str = "------------------------ea3bbcf87c101592";

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


fn get_image_data(file: PathBuf) -> Result<Vec<u8>, AppError> {
    let mut data = Vec::new();
    write!(&mut data, "--{}\r\n", BOUNDARY)?;
    write!(
        &mut data,
        "{}",
        &format!(
            "Content-Disposition: form-data; name=\"smfile\"; filename=\"{}\"\r\n",
            file.to_str().expect("Cannot convert pathbuf to str"))
    )?;
    write!(&mut data, "Content-Type: image/jpeg\r\n")?;
    write!(&mut data, "\r\n")?;

    let mut f = File::open(file)?;
    f.read_to_end(&mut data)?;

    write!(&mut data, "\r\n")?;
    write!(&mut data, "--{}--\r\n", BOUNDARY)?;

    Ok(data)
}

pub async fn upload(context: &Context) -> Result<(), AppError> {
    let Context { file, .. } = context;
    let client = Client::new();
    let image_data = get_image_data(file.to_path_buf())?;
    let mut req = Request::new(Body::from(image_data));
    req.headers_mut().insert(
        CONTENT_TYPE,
        format!("multipart/form-data; boundary={}", BOUNDARY)
            .parse()
            .expect("Cannot parse header"),
    );
    *req.method_mut() = Method::POST;
    *req.uri_mut() = "http://bayfiles.com/api/upload".parse().expect("Cannot parse uri");
    let response = client.request(req).compat().await?;
    let body = response.body();
    println!("body: {:?}", body);

    Ok(())
}
