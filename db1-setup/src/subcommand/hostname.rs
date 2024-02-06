use crate::{
    error::Error,
    platform::Platform,
};


pub(crate) fn subcommand_hostname(_platform: &Platform) -> Result<(), Error> {
    println!("<< SUBCOMMAND: HOSTNAME >>");
    
    Ok(())
}
