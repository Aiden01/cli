#![feature(async_await)]

use structopt::StructOpt;

mod app;
mod error;

use app::{upload, Context};

#[tokio::main]
async fn main() {
    let context = Context::from_args();
    let res = upload(&context).await;
    println!("{:?}", res);
}

