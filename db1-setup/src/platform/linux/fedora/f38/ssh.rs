mod configure;
mod verify;


use configure::configure;
use crate::platform::ssh::Platform;
use lazy_static::lazy_static;
use verify::verify;


lazy_static! {
    pub(super) static ref PLATFORM: Platform = Platform {
        configure,
        verify,
    };
}
