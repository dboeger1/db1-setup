use crate::error::Error;
use std::path::PathBuf;


#[derive(Clone)]
pub(crate) struct Platform {
    pub(crate) destination: Option<PathBuf>,
    pub(crate) source: Option<PathBuf>,
    pub(crate) install: fn() -> Result<(), Error>,
    pub(crate) verify: fn() -> Result<(), Error>,
}
