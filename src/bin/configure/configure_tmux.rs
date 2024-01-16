fn configure_tmux() {
    let file = PathBuf::from("~/.tmux.conf");
    if file.exists() {
        // error; tmux conf already exists
    }
    // copy
}
