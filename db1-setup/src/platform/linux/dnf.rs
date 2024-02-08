use crate::error::Error;
use prun::prun;
use std::process::{
    Command,
    Output,
};


pub(crate) fn copr_enable(copr: &str) -> Result<(), Error> {
    let mut dnf_command = Command::new("dnf");
    dnf_command.args([
        "copr",
        "enable",
        "-y",
        copr,
    ]);

    let output: Output;
    match prun(&mut dnf_command, false) {
        Ok(output_inner) => output = output_inner,
        Err(error) => return Err(Error {
            message: "Error running dnf command.".to_string(),
            source: Some(Box::new(error)),
        }),
    };

    if !output.status.success() {
        return Err(Error {
            message: "dnf command failed.".to_string(),
            source: None,
        });
    }

    Ok(())
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

    let output: Output;
    match prun(&mut dnf_command, false) {
        Ok(output_inner) => output = output_inner,
        Err(error) => return Err(Error {
            message: "Error running dnf command.".to_string(),
            source: Some(Box::new(error)),
        }),
    };

    if !output.status.success() {
        return Err(Error {
            message: "dnf command failed.".to_string(),
            source: None,
        });
    }

    Ok(())
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
