mod build;
mod clean;
mod package_args;
mod package_error;


use build::build;
use clap::Parser;
use clean::clean;
use package_args::PackageArgs;
use std::{
    error::Error,
    process::ExitCode,
};


fn main() -> ExitCode {
    match PackageArgs::parse().clean {
        true => {
            match clean() {
                Ok(_) => ExitCode::SUCCESS,
                Err(error) => {
                    eprintln!("{}", error);
                    if let Some(source) = error.source() {
                        eprintln!("{}", source);
                    }
                    ExitCode::FAILURE
                }
            }
        },
        false => {
            match build() {
                Ok(_) => ExitCode::SUCCESS,
                Err(error) => {
                    eprintln!("{}", error);
                    if let Some(source) = error.source() {
                        eprintln!("source: {}", source);
                    }
                    ExitCode::FAILURE
                }
            }
        },
    }
}
