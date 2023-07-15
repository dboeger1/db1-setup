use std::process::{Command, Stdio};


fn main() {
    // Install dependencies. This should probably be broken up by domain.
    install_dependencies();

    // Install configuration files.
    install_dotfiles();
}


fn install_dependencies() {
    let dnf = Command::new("dnf")
        .args([
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

              // Python
              "python",

              // Javascript
              "npm",

              // RPM
              "rpm-build",
              "rpm-devel",
              "rpmdevtools",
              "rpmlint"
        ])
        .output()
        .unwrap();
    println!("{}", std::str::from_utf8(&dnf.stdout).unwrap());

    install_rustup();
}


fn install_rustup() {
    // Retrieve rustup installation script.
    let curl = Command::new("curl")
        .args([
            // Enable/disable protocols.
            "--proto", "=https",

            // Use TLSv1.2 or greater.
            "--tlsv1.2",

            // Silent mode.
            "-s",

            // Show error even when -s is used.
            "-S",

            // Fail fast with no output on HTTP errors.
            "-f",

            // URL
            "https://sh.rustup.rs"
        ])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    // Install rustup.
    let sh = Command::new("sh")
        .args([
              // Read commands from the standard input.
              "-s",

              // Denotes the end of the options and start of standard input.
              "--",

              // Disable confirmation prompt.
              "-y"
        ])
        .stdin(Stdio::from(curl.stdout.unwrap()))
        .output()
        .unwrap();
    println!("{}", std::str::from_utf8(&sh.stdout).unwrap());
}


fn install_dotfiles() {
    install_tmux_config();
    install_neovim_config();
}


fn install_tmux_config() {
}


fn install_neovim_config() {
}
