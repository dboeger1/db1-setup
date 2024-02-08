mod install;
mod verify;


use crate::platform::rust::Platform;
use lazy_static::lazy_static;
use install::install;
use verify::verify;


lazy_static! {
    pub(super) static ref PLATFORM: Platform = Platform {
        install,
        verify,
    };
}


//rustup
//rustup_init()
