use std::fmt;

#[derive(Debug)]
pub enum Error {
    InvalidNumberOfArguments,
    InvalidOption,
    ParseError(Box<dyn std::error::Error>),
}

impl<T: 'static + std::error::Error> From<T> for Error {
    fn from(error: T) -> Self {
        Error::ParseError(Box::new(error))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InvalidNumberOfArguments => write!(f, "Invalid number of arguments"),
            Error::InvalidOption => write!(f, "Invalid option"),
            Error::ParseError(error) => error.fmt(f),
        }
    }
}
