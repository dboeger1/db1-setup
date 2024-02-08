pub(crate) mod configure;
pub(crate) mod verify;


use configure::configure;
use crate::platform::ssh::Platform;
use lazy_static::lazy_static;
use verify::verify;


lazy_static! {
    pub(crate) static ref PLATFORM: Platform = Platform {
        configure,
        verify,
    };
}
