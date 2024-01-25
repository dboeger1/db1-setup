use crate::platform::Platform;
use lazy_static::lazy_static;


lazy_static! {
    pub(crate) static ref PLATFORM: Platform = Platform {
        stuff: || println!("f38 stuff"),
    };
}
