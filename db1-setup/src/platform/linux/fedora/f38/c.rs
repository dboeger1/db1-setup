pub(crate) mod install;
pub(crate) mod verify;


use crate::platform::c::Platform;
use lazy_static::lazy_static;
use install::install;
use verify::verify;


lazy_static! {
    pub(crate) static ref PLATFORM: Platform = Platform {
        install,
        verify,
    };
}


//cmake
//gcc
//make
