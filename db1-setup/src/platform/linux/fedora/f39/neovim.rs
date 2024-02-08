pub(crate) mod install;
pub(crate) mod verify;


use crate::{
    HOME_DIR,
    platform::{
        linux::INSTALL_DIR,
        neovim::Platform,
    },
};
use lazy_static::lazy_static;
use install::install;
use verify::verify;


lazy_static! {
    pub(crate) static ref PLATFORM: Platform = Platform {
        destination: Some(HOME_DIR.join(".config/nvim")),
        source: Some(INSTALL_DIR.join("neovim")),
        install,
        verify,
    };
}


//neovim
