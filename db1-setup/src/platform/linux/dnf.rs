use crate::error::Error;
use prun::prun;
use std::process::Command;


pub(crate) fn copr_enable(copr: &str) -> Result<(), Error> {
    let mut dnf_command = Command::new("dnf");
    dnf_command.args([
        "copr",
        "enable",
        "-y",
        copr,
    ]);

    prun(&mut dnf_command, false)
        .map_err(|error| Error {
            message: "Error running dnf command.".to_string(),
            source: Some(Box::new(error)),
        })
}

pub(crate) fn install<I>(packages: I) -> Result<(), Error>
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let mut dnf_command = Command::new("dnf");
    dnf_command.args([
        "install",
        "-y",
    ]);
    for package in packages {
        dnf_command.arg(package.as_ref());
    }

    prun(&mut dnf_command, false)
        .map_err(|error| Error {
            message: "Error running dnf command.".to_string(),
            source: Some(Box::new(error)),
        })
}

//pub(crate) fn is_installed(package: &str) -> Result<bool, Error> {
//    let mut dnf_command = Command::new("dnf");
//    dnf_command.args([
//        "info",
//        "--installed",
//        package,
//    ]);
//
//    prun(&mut dnf_command, false)
//        .map_err(|error| Error {
//            message: "Error running dnf command.".to_string(),
//            source: Some(Box::new(error)),
//        })?;
//}
