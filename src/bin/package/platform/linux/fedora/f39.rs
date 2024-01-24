use crate::platform::Platform;

pub(crate) const PLATFORM: PlatformFedora39 = PlatformFedora39 {};

pub(crate) struct PlatformFedora39 {}

impl Platform for PlatformFedora39 {
    fn stuff(&self) {
        println!("f39 stuff");
    }
}
