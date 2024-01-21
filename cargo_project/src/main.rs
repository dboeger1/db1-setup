mod error;
mod platform;
mod source_destination;


use platform::PLATFORM;
use std::process::ExitCode;


#[macro_use]
extern crate lazy_static;


pub(crate) const CARGO_NAME: &str = env!("CARGO_PKG_NAME");


fn main() -> ExitCode {
    match PLATFORM.as_ref() {
        Some(platform_data) => {
            if let Err(error) = platform_data.install_packages() {
                eprintln!("{}", error);
                if let Some(source) = error.source {
                    eprintln!("{}", source);
                }
                return ExitCode::FAILURE;
            }

            println!("tmux paths:");
            println!(
                "\t{}",
                platform_data
                    .get_tmux_paths()
                    .unwrap()
                    .source
                    .to_string_lossy(),
            );
            println!(
                "\t{}",
                platform_data
                    .get_tmux_paths()
                    .unwrap()
                    .destination
                    .to_string_lossy(),
            );

            println!("neovim paths:");
            println!(
                "\t{}",
                platform_data
                    .get_neovim_paths()
                    .unwrap()
                    .source
                    .to_string_lossy(),
            );
            println!(
                "\t{}",
                platform_data
                    .get_neovim_paths()
                    .unwrap()
                    .destination
                    .to_string_lossy(),
            );

            ExitCode::SUCCESS
        },
        _ => {
            eprintln!("unrecognized platform");

            ExitCode::FAILURE
        },
    }
}
