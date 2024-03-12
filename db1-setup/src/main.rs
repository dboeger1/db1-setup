use db1_setup::execute;
use std::{
    error::Error,
    process::ExitCode,
};


fn main() -> ExitCode {
    if let Err(error) = execute() {
        eprintln!("Error: {error}");
        if let Some(source) = error.source() {
            eprintln!("Source: {source}");
        }
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
