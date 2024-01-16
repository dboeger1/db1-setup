use crate::configure_error::ConfigureError;
use std::{
    process::Command,
    str::from_utf8,
};


pub(crate) fn install_dependencies() -> Result<(), ConfigureError> {
    let mut dnf_command = Command::new("dnf");
    dnf_command.args([
          "install",

          // Utilities
          "bash",
          "coreutils",
          "diffutils",
          "fd-find",
          "git",
          "patch",
          "ripgrep",
          "tree",
          "unzip",
          "wget",

          // Applications
          "neovim",

          // C
          "cmake",
          "gcc",
          "make",

          // C++
          "gcc-c++",

          // Rust
          "rustup",

          // Python
          "python",

          // Javascript
          "npm",

          // RPM
          "rpm-build",
          "rpm-devel",
          "rpmdevtools",
          "rpmlint",
    ]);
    let mut dnf_string = dnf_command
        .get_program()
        .to_os_string();
    dnf_command
        .get_args()
        .for_each(|arg| dnf_string.push(format!(
            " {}",
            arg.to_string_lossy(),
        )));

    let dnf_output = dnf_command
        .output()
        .map_err(|error| ConfigureError {
            message: format!(
                "failed to execute dnf command \"{}\"",
                dnf_string.to_string_lossy(),
            ),
            source: Some(error),
        })?;
    if !dnf_output.status.success() {
        // command
        eprintln!("==== command ====");
        eprintln!("{}", dnf_string.to_string_lossy());

        // exit code
        let dnf_exit_code = dnf_output
            .status
            .code();
        eprintln!("==== exit code ====");
        eprintln!(
            "{}",
            dnf_exit_code.map_or_else(
                || "<failed to retrieve>".to_string(),
                |status| status.to_string()
            )
        );

        // stdout
        let dnf_stdout = from_utf8(&dnf_output.stdout);
        eprintln!("==== stdout ====");
        if dnf_stdout.is_ok() {
            eprintln!("{}", dnf_stdout.unwrap_or("<failed to retrieve>"));
        }

        // stderr
        let dnf_stderr = from_utf8(&dnf_output.stderr);
        eprintln!("==== stderr ====");
        if dnf_stderr.is_ok() {
            eprintln!("{}", dnf_stderr.unwrap_or("<failed to retrieve>"));
        }

        return Err(ConfigureError {
            message: format!(
                "dnf command \"{}\" failed",
                dnf_string.to_string_lossy(),
            ),
            source: None,
        });
    }

    Ok(())
}
