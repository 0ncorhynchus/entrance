use std::fmt;

#[derive(Debug)]
pub enum Error {
    InvalidNumberOfArguments,
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
            Error::InvalidNumberOfArguments => write!(f, "Invalid number of arguments"),
            Error::Others(error) => error.fmt(f),
        }
    }
}
