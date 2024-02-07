use crate::{
    error::Error,
    platform::Platform,
};


pub(crate) fn subcommand_configure(_platform: &Platform) -> Result<(), Error> {
    println!("=== SUBCOMMAND: HOSTNAME CONFIGURE ===");

    Ok(())
}
