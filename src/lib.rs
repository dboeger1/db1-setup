use lazy_static::lazy_static;
use std::{
    env::var,
    path::PathBuf,
};


lazy_static! {
    pub static ref OS_INFO: os_info::Info =
        os_info::get();

    pub static ref HOME_DIR: PathBuf =
        PathBuf::from(var("HOME").unwrap());
}
