mod fedora;


use crate::{
    error::Error,
    platform::Platform,
};
use dboeger1_dotfiles::{
    CARGO_NAME,
    OS_INFO,
};
use lazy_static::lazy_static;
use os_info::Type;
use print_command::run_and_print;
use std::{
    path::PathBuf,
    process::Command,
};



lazy_static! {
    pub(crate) static ref PLATFORM: Option<&'static Platform> =
        match OS_INFO.os_type() {
            Type::Fedora => *fedora::PLATFORM,
            _ => None,
        };

    // User paths.
    pub(crate) static ref INSTALL_DIR: PathBuf =
        PathBuf::from(format!("/opt/{}", CARGO_NAME));
}


pub(crate) fn dnf_install<I>(packages: I) -> Result<(), Error>
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let mut dnf_command = Command::new("dnf");
    dnf_command.arg("install");
    packages
        .into_iter()
        .for_each(|package| {
            dnf_command.arg(package.as_ref());
        });

    run_and_print(&mut dnf_command, false)
        .map_err(|error| Error {
            message: "".to_string(),
            source: Some(Box::new(error) as Box<dyn std::error::Error>),
        })
}
