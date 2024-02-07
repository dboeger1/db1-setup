mod error;
mod platform;
mod subcommand;


use clap::Parser;
use error::Error;
use lazy_static::lazy_static;
use os_info::{
    get,
    Info,
};
use platform::{
    Platform,
    PLATFORM,
};
use std::{
    env::var,
    path::PathBuf,
};
use subcommand::{
    execute_subcommand,
    Subcommand,
};


pub const CARGO_NAME: &str = env!("CARGO_PKG_NAME");

lazy_static! {
    pub static ref OS_INFO: Info = get();
    pub static ref HOME_DIR: PathBuf = PathBuf::from(var("HOME").unwrap());
}


#[derive(Parser)]
#[command(name = env!("CARGO_CRATE_NAME"))]
#[command(version)]
pub struct Args {
    #[command(subcommand)]
    pub subcommand: Subcommand,
}


pub fn execute(args: &Args) -> Result<(), Error> {
    // Get platform data.
    let platform: &Platform;
    match PLATFORM.as_ref() {
        Some(platform_data) => platform = platform_data,
        None => return Err(Error {
            message: "Unrecognized platform. Aborting.".to_string(),
            source: None,
        }),
    }

    // Execute subcommand.
    execute_subcommand(platform, &args.subcommand)
}
