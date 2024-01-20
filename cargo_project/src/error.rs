use std::{
    error::Error,
    fmt::Display,
    io,
};


#[derive(Debug)]
pub(crate) struct ConfigureError {
    pub(crate) message: String,
    pub(crate) source: Option<io::Error>,
}

impl Display for ConfigureError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ConfigureError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|error| error as &(dyn Error + 'static))
    }
}
