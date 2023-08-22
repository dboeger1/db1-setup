#![allow(dead_code)]


use std::path::PathBuf;


#[derive(Debug)]
struct PathAB {
    a: PathBuf,
    b: PathBuf,
}

fn main() {
    // Output from "rpm -qlp <.rpm>".
    let lines = r#"
        /opt/dboeger1-dotfiles/neovim/init.lua
        /opt/dboeger1-dotfiles/neovim/lazy-lock.json
        /opt/dboeger1-dotfiles/neovim/lua/.luarc.json
        /opt/dboeger1-dotfiles/neovim/lua/plugins/lazy/plugin_cmp.lua
        /opt/dboeger1-dotfiles/neovim/lua/plugins/lazy/plugin_kanagawa.lua
        /opt/dboeger1-dotfiles/neovim/lua/plugins/lazy/plugin_lualine.lua
        /opt/dboeger1-dotfiles/neovim/lua/plugins/lazy/plugin_mason.lua
        /opt/dboeger1-dotfiles/neovim/lua/plugins/lazy/plugin_nightfox.lua
        /opt/dboeger1-dotfiles/neovim/lua/plugins/lazy/plugin_snippy.lua
        /opt/dboeger1-dotfiles/neovim/lua/plugins/lazy/plugin_telescope.lua
        /opt/dboeger1-dotfiles/neovim/lua/plugins/lazy/plugin_telescope_fzf_native.lua
        /opt/dboeger1-dotfiles/neovim/lua/plugins/lazy/plugin_treesitter.lua
        /opt/dboeger1-dotfiles/neovim/lua/plugins/plugin_netrw.lua
        /opt/dboeger1-dotfiles/neovim/lua/settings/indentation.lua
        /opt/dboeger1-dotfiles/neovim/lua/settings/information.lua
        /opt/dboeger1-dotfiles/neovim/lua/settings/key_maps.lua
        /opt/dboeger1-dotfiles/neovim/lua/settings/tabs.lua
        /opt/dboeger1-dotfiles/neovim/lua/settings/themes.lua
        /opt/dboeger1-dotfiles/neovim/lua/settings/windows.lua
        /opt/dboeger1-dotfiles/tmux/.tmux.conf
    "#;

    lines
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            match line.split_once("dboeger1-dotfiles/") {
                None => None,
                Some((_, suffix)) => {
                    let destination: String;
                    if suffix.starts_with("neovim/") {
                        destination = suffix.replacen(
                            "neovim",
                            "/root/.config/nvim",
                            1);
                    } else if suffix.starts_with("tmux/") {
                        destination = suffix.replacen(
                            "tmux",
                            "/root",
                            1);
                    } else {
                        return None;
                    }

                    Some(PathAB {
                        a: PathBuf::from(line),
                        b: PathBuf::from(destination),
                    })
                },
            }
        })
        .for_each(|file_copy| {
            println!("Copying File");
            println!("\tSource: \"{}\"", file_copy.a.to_string_lossy());
            println!("\tDestination: \"{}\"", file_copy.b.to_string_lossy());

            if file_copy.a.try_exists().unwrap() != true {
                println!(
                    "\t\tSource does not exist: \"{}\"",
                    file_copy.a.to_string_lossy()
                );
                return;
            }

            if file_copy.b.try_exists().unwrap() == true {
                println!(
                    "\t\tDestination already exists: \"{}\"",
                    file_copy.b.to_string_lossy()
                );
                return;
            }

            //let bytes_copied = fs::copy(file_copy.a, file_copy.b).unwrap();
            //println!("\t\tbytes_copied: {}", bytes_copied);
        });
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
