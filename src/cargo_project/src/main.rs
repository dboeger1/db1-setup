#![allow(dead_code)]


use std::{
    path::PathBuf,
    str::FromStr,
};


#[derive(Debug)]
struct PathAB {
    a: PathBuf,
    b: PathBuf,
}

fn main() {
    let file_copies: [PathAB; 3] = [
        PathAB {
            a: PathBuf::from_str("tmux/.tmux.conf").unwrap(),
            b: PathBuf::from_str("/root/tmux/.tmux.conf").unwrap(),
        },
        PathAB {
            a: PathBuf::from_str("neovim/init.lua").unwrap(),
            b: PathBuf::from_str("/root/.config/nvim/init.lua").unwrap(),
        },
        PathAB {
            a: PathBuf::from_str("neovim/lazy-lock.json").unwrap(),
            b: PathBuf::from_str("/root/.config/nvim/lazy-lock.json").unwrap(),
        },
    ];

    for file_copy in file_copies {
        println!("Copying File");

        if file_copy.a.try_exists().unwrap() != true {
            println!(
                "\tSource does not exist: \"{}\"",
                file_copy.a.to_string_lossy()
            );
            continue;
        }

        if file_copy.b.try_exists().unwrap() == true {
            println!(
                "\tDestination already exists: \"{}\"",
                file_copy.b.to_string_lossy()
            );
            continue;
        }

        println!("    Source: \"{}\"", file_copy.a.to_string_lossy());
        println!("    Destination: \"{}\"", file_copy.b.to_string_lossy());
    }
}


//fn install_dependencies() {
//    let dnf = Command::new("dnf")
//        .args([
//              "install",
//
//              // Utilities
//              "bash",
//              "coreutils",
//              "diffutils",
//              "fd-find",
//              "git",
//              "patch",
//              "ripgrep",
//              "tree",
//              "unzip",
//              "wget",
//
//              // Applications
//              "neovim",
//
//              // C
//              "cmake",
//              "gcc",
//              "make",
//
//              // C++
//              "gcc-c++",
//
//              // Python
//              "python",
//
//              // Javascript
//              "npm",
//
//              // RPM
//              "rpm-build",
//              "rpm-devel",
//              "rpmdevtools",
//              "rpmlint"
//        ])
//        .output()
//        .unwrap();
//    println!("{}", std::str::from_utf8(&dnf.stdout).unwrap());
//
//    install_rustup();
//}


//fn install_rustup() {
//    // Retrieve rustup installation script.
//    let curl = Command::new("curl")
//        .args([
//            // Enable/disable protocols.
//            "--proto", "=https",
//
//            // Use TLSv1.2 or greater.
//            "--tlsv1.2",
//
//            // Silent mode.
//            "-s",
//
//            // Show error even when -s is used.
//            "-S",
//
//            // Fail fast with no output on HTTP errors.
//            "-f",
//
//            // URL
//            "https://sh.rustup.rs"
//        ])
//        .stdout(Stdio::piped())
//        .spawn()
//        .unwrap();
//
//    // Install rustup.
//    let sh = Command::new("sh")
//        .args([
//              // Read commands from the standard input.
//              "-s",
//
//              // Denotes the end of the options and start of standard input.
//              "--",
//
//              // Disable confirmation prompt.
//              "-y"
//        ])
//        .stdin(Stdio::from(curl.stdout.unwrap()))
//        .output()
//        .unwrap();
//    println!("{}", std::str::from_utf8(&sh.stdout).unwrap());
//}
