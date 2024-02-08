use crate::{
    error::Error,
    hostname::configure::Args,
};


pub(crate) fn configure(_args: &Args) -> Result<(), Error> {
    println!("<< F39 HOSTNAME CONFIGURE >>");

    Ok(())
}
