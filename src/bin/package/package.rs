use crate::{
    error::Error,
    platform::PLATFORM,
};


pub(crate) fn package() -> Result<(), Error> {
    PLATFORM
        .as_ref()
        .unwrap()
        .stuff();

    Ok(())
}
