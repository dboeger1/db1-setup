mod args;
mod clean;
mod error;
mod package;
mod platform;
mod values;


use args::Args;
use clap::Parser;
use crate::{
    clean::clean,
    package::package,
};
use std::{
    error::Error,
    process::ExitCode,
};


pub fn main() -> ExitCode {
    let args = Args::parse();

    let operation = match args.clean {
        false => package,
        true => clean,
    };

    if let Err(error) = operation() {
        eprintln!("{}", error);

        if let Some(source) = error.source() {
            eprintln!("source: {}", source);
        }

        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
