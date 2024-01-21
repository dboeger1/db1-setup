use crate::{
    platform::{
        Platform,
        INSTALL_DIR,
        HOME_DIR,
    },
    source_destination::SourceDestination, error::ConfigureError,
};


pub(crate) const PLATFORM: PlatformFedora39 = PlatformFedora39 {};

pub(crate) struct PlatformFedora39 {}

fn install_packages() -> Result<(), ConfigureError> {
    println!("f39 install_packages() inner function");
    Ok(())
}

impl Platform for PlatformFedora39 {
    fn get_neovim_paths(&self) -> Option<SourceDestination> {
        Some(SourceDestination {
            source: INSTALL_DIR.join("neovim"),
            destination: HOME_DIR.join(".config/nvim"),
        })
    }

    fn get_tmux_paths(&self) -> Option<SourceDestination> {
        Some(SourceDestination {
            source: INSTALL_DIR.join("tmux/.tmux.conf"),
            destination: HOME_DIR.join(".tmux.conf"),
        })
    }

    fn get_install_packages(&self) -> Option<
            fn() -> Result<(), crate::error::ConfigureError>
        > {
       Some(install_packages)
    }

    //fn install_packages() -> Result<(), crate::error::ConfigureError> {
    //    println!("Installing useful packages...");

    //    let mut dnf_command = Command::new("dnf");
    //    dnf_command.args([
    //          "install",

    //    ]);
    //    let mut dnf_string = dnf_command
    //        .get_program()
    //        .to_os_string();
    //    dnf_command
    //        .get_args()
    //        .for_each(|arg| dnf_string.push(format!(
    //            " {}",
    //            arg.to_string_lossy(),
    //        )));

    //    let dnf_output = dnf_command
    //        .output()
    //        .map_err(|error| ConfigureError {
    //            message: format!(
    //                "failed to execute dnf command \"{}\"",
    //                dnf_string.to_string_lossy(),
    //            ),
    //            source: Some(error),
    //        })?;
    //    if !dnf_output.status.success() {
    //        // command
    //        eprintln!("==== command ====");
    //        eprintln!("{}", dnf_string.to_string_lossy());

    //        // exit code
    //        let dnf_exit_code = dnf_output
    //            .status
    //            .code();
    //        eprintln!("==== exit code ====");
    //        eprintln!(
    //            "{}",
    //            dnf_exit_code.map_or_else(
    //                || "<failed to retrieve>".to_string(),
    //                |status| status.to_string()
    //            )
    //        );

    //        // stdout
    //        let dnf_stdout = from_utf8(&dnf_output.stdout);
    //        eprintln!("==== stdout ====");
    //        if dnf_stdout.is_ok() {
    //            eprintln!("{}", dnf_stdout.unwrap_or("<failed to retrieve>"));
    //        }

    //        // stderr
    //        let dnf_stderr = from_utf8(&dnf_output.stderr);
    //        eprintln!("==== stderr ====");
    //        if dnf_stderr.is_ok() {
    //            eprintln!("{}", dnf_stderr.unwrap_or("<failed to retrieve>"));
    //        }

    //        return Err(ConfigureError {
    //            message: format!(
    //                "dnf command \"{}\" failed",
    //                dnf_string.to_string_lossy(),
    //            ),
    //            source: None,
    //        });
    //    }

    //    println!("Done.");
    //    Ok(())
    //}
}
