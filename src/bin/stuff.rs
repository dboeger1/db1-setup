use std::path::PathBuf;

use dboeger1_dotfiles::*;

fn main() {
    let node1 = FileTreeNode {
        directory: PathBuf::from("/a/b"),
        files: vec![
            PathBuf::from("/a/b/file3.txt"),
            PathBuf::from("/a/b/file4.txt"),
        ],
        children: vec![],
    };

    let node2 = FileTreeNode {
        directory: PathBuf::from("/a/c"),
        files: vec![
            PathBuf::from("/a/c/file5.txt"),
            PathBuf::from("/a/c/file6.txt"),
        ],
        children: vec![],
    };

    let node3 = FileTreeNode {
        directory: PathBuf::from("/a"),
        files: vec![
            PathBuf::from("/a/file1.txt"),
            PathBuf::from("/a/file2.txt"),
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
