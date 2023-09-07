mod build;
mod clean;


use build::build;
use clap::Parser;
use clean::clean;
use std::{
    error::Error,
    fmt::Display,
    io,
    process::ExitCode,
};


#[derive(Debug)]
struct PackageError {
    message: String,
    source: Option<io::Error>,
}

impl Display for PackageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for PackageError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|error| error as &(dyn Error + 'static))
    }
}


#[derive(Parser)]
#[command(name = env!("CARGO_CRATE_NAME"))]
#[command(version)]
struct Args {
    #[arg(short, long)]
    clean: bool,
}

fn main() -> ExitCode {
    match Args::parse().clean {
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
