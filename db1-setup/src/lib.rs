mod error;
mod platform;


use lazy_static::lazy_static;
use os_info::{
    get,
    Info,
};
use platform::{
    Strategy,
    STRATEGY,
};
use std::{
    env::var,
    path::PathBuf,
};

pub use error::Error;


pub const CARGO_NAME: &str = env!("CARGO_PKG_NAME");

lazy_static! {
    pub static ref OS_INFO: Info = get();
    pub(crate) static ref HOME_DIR: PathBuf =
        PathBuf::from(var("HOME").unwrap());
}

pub fn execute() -> Result<(), Error> {
    let execute_strategy: Strategy;
    match STRATEGY.as_ref() {
        Some(&strategy) => execute_strategy = strategy,
        None => return Err(Error {
            message: "Unrecognized platform. Aborting.".to_string(),
            source: None,
        }),
    }

    execute_strategy()
}
