use crate::{
    error::Error,
    subcommand::ssh::subcommand::configure::Args,
};


#[derive(Clone)]
pub(crate) struct Platform {
    pub(crate) configure: fn(&Args) -> Result<(), Error>,
    pub(crate) verify: fn() -> Result<(), Error>,
}
