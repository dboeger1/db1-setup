use std::path::{
    Path,
    PathBuf,
};


pub struct SourceDestination {
    pub source: PathBuf,
    pub destination: PathBuf,
}

#[derive(Debug)]
pub struct FileTreeNode {
    pub directory: PathBuf,
    pub files: Vec<PathBuf>,
    pub children: Vec<FileTreeNode>,
}

impl FileTreeNode {
    pub fn iter(&self) -> FileTreeNodeIterator {
        FileTreeNodeIterator {
            node: &self,
            files_index: 0,
            children_index: 0,
            child_iterator: None,
        }
    }
}

pub struct FileTreeNodeIterator<'a> {
    pub node: &'a FileTreeNode,
    pub files_index: usize,
    pub children_index: usize,
    pub child_iterator: Option<Box<FileTreeNodeIterator<'a>>>,
}

impl<'a> Iterator for FileTreeNodeIterator<'a> {
    type Item = &'a Path;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(file) = self
            .node
            .files
            .get(self.files_index) {
            self.files_index += 1;
            return Some(file.as_path());
        }

        while self.children_index < self.node.children.len() {
            if self.child_iterator.is_none() {
                self.child_iterator = Some(Box::new(self
                    .node
                    .children
                    .get(self.children_index)
                    .unwrap()
                    .iter()
                ));
            }

            if let Some(next_item) = self
                .child_iterator
                .as_mut()
                .unwrap()
                .next() {
                return Some(next_item);
            }

            self.child_iterator = None;
            self.children_index += 1;
        }

        None
    }
}
