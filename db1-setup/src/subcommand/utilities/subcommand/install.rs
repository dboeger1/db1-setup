use crate::{
    error::Error,
    platform::Platform,
};


pub(crate) fn subcommand_install(_platform: &Platform) -> Result<(), Error> {
    println!("=== SUBCOMMAND: UTILITIES INSTALL ===");

    Ok(())
}
