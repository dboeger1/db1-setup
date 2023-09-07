use std::{path::PathBuf, ffi::OsString};

use dboeger1_dotfiles::*;

fn main() {
    let node1 = FileTreeNode {
        directory: PathBuf::from("/a/b"),
        files: vec![
            OsString::from("file1.txt"),
            OsString::from("file2.txt"),
        ],
        children: vec![],
    };

    let node2 = FileTreeNode {
        directory: PathBuf::from("/a/c"),
        files: vec![
            OsString::from("file3.txt"),
            OsString::from("file4.txt"),
        ],
        children: vec![],
    };

    let node3 = FileTreeNode {
        directory: PathBuf::from("/a"),
        files: vec![
            OsString::from("file5.txt"),
            OsString::from("file6.txt"),
        ],
        children: vec![
            node1,
            node2,
        ],
    };

    node3
        .iter()
        .for_each(|file| println!("{}", file.to_string_lossy()));
}
