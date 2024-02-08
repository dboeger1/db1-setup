use crate::error::Error;


#[derive(Clone)]
pub(crate) struct Platform {
    pub(crate) install: fn() -> Result<(), Error>,
    pub(crate) verify: fn() -> Result<(), Error>,
}
