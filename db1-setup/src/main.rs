use clap::Parser;
use db1_setup::{
    Args,
    execute,
};
use std::{
    error::Error,
    process::ExitCode,
};


fn main() -> ExitCode {
    // Parse arguments.
    let args = Args::parse();

    // Execute command logic.
    if let Err(error) = execute(&args) {
        eprintln!("Error: {error}");
        if let Some(source) = error.source() {
            eprintln!("Source: {source}");
        }
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
