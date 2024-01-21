mod error;
mod platform;
mod source_destination;


use platform::PLATFORM;
use std::process::ExitCode;


#[macro_use]
extern crate lazy_static;


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

            println!(
                "tmux paths: {:#?}",
                platform_data.get_tmux_paths(),
            );

            println!(
                "nvim paths: {:#?}",
                platform_data.get_neovim_paths(),
            );

            ExitCode::SUCCESS
        },
        _ => {
            eprintln!("unrecognized platform");

            ExitCode::FAILURE
        },
    }
}
