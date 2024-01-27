use lazy_static::lazy_static;
use os_info::{
    get,
    Info,
};
use std::{
    env::var,
    path::PathBuf,
};


pub const CARGO_NAME: &str = env!("CARGO_PKG_NAME");

lazy_static! {
    pub static ref OS_INFO: Info = get();
    pub static ref HOME_DIR: PathBuf = PathBuf::from(var("HOME").unwrap());
}
