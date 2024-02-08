use crate::{
    error::Error,
    hostname::configure::Args,
};


pub(super) fn configure(_args: &Args) -> Result<(), Error> {
    println!("<< F38 HOSTNAME CONFIGURE >>");

    Ok(())
}
