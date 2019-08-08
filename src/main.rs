#![feature(async_await, try_trait)]

use futures::compat::Compat;
use futures::future::{FutureExt, TryFutureExt};
use structopt::StructOpt;

mod app;
mod error;

use app::{upload, Context};

fn main() {
    let context = Context::from_args();
    let upload_task = async {
        let url = upload(context).await?;
        println!("Url: {:?}", url);
        Ok(())
    }
        .map_err(|err: error::AppError| eprintln!("Couldn't upload: {:?}", err))
        .boxed();
    tokio::run(Compat::new(upload_task));
}
