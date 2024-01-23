use std::path::PathBuf;


#[derive(Debug)]
pub(crate) struct SourceDestination {
    pub(crate) source: PathBuf,
    pub(crate) destination: PathBuf,
}
