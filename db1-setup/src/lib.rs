mod error;
mod platform;


use platform::strategy;

pub use error::Error;


pub const CARGO_NAME: &str = env!("CARGO_PKG_NAME");


pub fn execute() -> Result<(), Error> {
    strategy().ok_or(Error {
        message: "Unrecognized platform. Aborting.".to_string(),
        source: None,
    })?()
}
