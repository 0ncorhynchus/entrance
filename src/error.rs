use std::fmt;

#[derive(Debug)]
pub enum Error {
    Others(Box<dyn std::error::Error>),
}

impl<T: 'static + std::error::Error> From<T> for Error {
    fn from(error: T) -> Self {
        Error::Others(Box::new(error))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Others(error) => error.fmt(f),
        }
    }
}
