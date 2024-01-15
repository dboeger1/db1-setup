use std::{
    path::PathBuf,
    process::Command,
};


fn main() {
    install_dependencies();
    configure_neovim();
    configure_tmux();
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
              "rpmlint"
        ])
        .output()
        .unwrap();
    println!("{}", std::str::from_utf8(&dnf.stdout).unwrap());
}

fn configure_neovim() {
    let file = PathBuf::from("~/.config/nvim");
    if file.exists() {
        // error; neovim conf already exists
    }
    // copy
}

fn configure_tmux() {
    let file = PathBuf::from("~/.tmux.conf");
    if file.exists() {
        // error; tmux conf already exists
    }
    // copy
}
