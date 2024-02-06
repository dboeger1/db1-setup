use crate::{
    error::Error,
    platform::Platform,
};


pub(crate) fn subcommand_incus(_platform: &Platform) -> Result<(), Error> {
    println!("<< SUBCOMMAND: INCUS >>");

    Ok(())
}
