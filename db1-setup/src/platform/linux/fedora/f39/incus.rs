mod install;
mod verify;


use crate::platform::incus::Platform;
use lazy_static::lazy_static;
use install::install;
use verify::verify;


lazy_static! {
    pub(super) static ref PLATFORM: Platform = Platform {
        install,
        verify,
    };
}


//    install(["dnf-command(copr)"])?;
//    copr_enable("ganto/lxc4")?;
//incus
