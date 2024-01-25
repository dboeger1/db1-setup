mod error;
mod platform;
mod source_destination;


use platform::PLATFORM;
use std::process::ExitCode;


pub(crate) const CARGO_NAME: &str = env!("CARGO_PKG_NAME");


fn main() -> ExitCode {
    if PLATFORM.is_none() {
        eprintln!("unrecognized platform");

        return ExitCode::FAILURE;
    }
    let platform_data = PLATFORM.unwrap();

    if let Some(install_packages) = platform_data.install_packages {
        if let Err(error) = install_packages() {
            eprintln!("{}", error);
            if let Some(source) = error.source {
                eprintln!("{}", source);
            }
            return ExitCode::FAILURE;
        }
    }

    println!("tmux paths:");
    println!(
        "\t{}",
        platform_data
            .tmux_paths
            .as_ref()
            .unwrap()
            .source
            .to_string_lossy(),
    );
    println!(
        "\t{}",
        platform_data
            .tmux_paths
            .as_ref()
            .unwrap()
            .destination
            .to_string_lossy(),
    );

    println!("neovim paths:");
    println!(
        "\t{}",
        platform_data
            .neovim_paths
            .as_ref()
            .unwrap()
            .source
            .to_string_lossy(),
    );
    println!(
        "\t{}",
        platform_data
            .neovim_paths
            .as_ref()
            .unwrap()
            .destination
            .to_string_lossy(),
    );

    ExitCode::SUCCESS
}
