use crate::platform::Platform;

pub(crate) const PLATFORM: PlatformFedora38 = PlatformFedora38 {};

pub(crate) struct PlatformFedora38 {}

impl Platform for PlatformFedora38 {
    fn stuff(&self) {
        println!("f38 stuff");
    }
}
