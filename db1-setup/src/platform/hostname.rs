use crate::{
    error::Error,
    subcommand::hostname::subcommand::configure::Args,
};


#[derive(Clone)]
pub(crate) struct Platform {
    pub(crate) configure: fn(&Args) -> Result<(), Error>,
    pub(crate) verify: fn() -> Result<(), Error>,
}
