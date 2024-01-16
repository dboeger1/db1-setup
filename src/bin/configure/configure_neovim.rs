fn configure_neovim() {
    let file = PathBuf::from("~/.config/nvim");
    if file.exists() {
        // error; neovim conf already exists
    }
    // copy
}
