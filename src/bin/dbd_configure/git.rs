use crate::{
    error::Error,
    platform::Platform,
};


pub(crate) fn subcommand_git(_platform: &Platform) -> Result<(), Error> {
    println!("<< SUBCOMMAND: GIT >>");
    
    Ok(())
}
