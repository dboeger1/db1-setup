mod clean;
mod error;
mod package;
mod platform;
mod values;


use clap::Parser;
use crate::{
    clean::clean,
    package::package,
};
use platform::PLATFORM;
use std::{
    error::Error,
    process::ExitCode,
};


#[derive(Parser)]
#[command(name = env!("CARGO_CRATE_NAME"))]
#[command(version)]
struct Args {
    #[arg(short, long)]
    clean: bool,
}


pub fn main() -> ExitCode {
    // Parse arguments.
    let args = Args::parse();

    // Clean.
    if args.clean {
        println!("Cleaning packages directory...");

        if let Err(error) = clean() {
            eprintln!("Error: {error}");
            if let Some(source) = error.source() {
                eprintln!("Source: {source}");
            }
            return ExitCode::FAILURE;
        }

        println!("Done.");
        return ExitCode::SUCCESS;
    }

    // Create packages.
    if let Some(platform_data) = PLATFORM.as_ref() {
        println!("Creating packages...");

        if let Err(error) = package(platform_data) {
            eprintln!("Error: {error}");
            if let Some(source) = error.source() {
                eprintln!("Source: {source}");
            }
            return ExitCode::FAILURE;
        }

        println!("Done.");
        return ExitCode::SUCCESS;
    }

    // Handle unsupported platform case.
    eprintln!("Unsupported platform. Aborting.");
    ExitCode::FAILURE
}
