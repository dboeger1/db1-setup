use crate::error::Error;
use print_command::run_and_print;
use std::process::Command;


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
            message: "command error".to_string(),
            source: Some(Box::new(error)),
        })
}
