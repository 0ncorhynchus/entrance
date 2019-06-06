use std::fmt;

#[derive(Debug)]
pub enum Error {
    InvalidNumberOfArguments,
    InvalidLongOption(String),
    InvalidShortOption(char),
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
            Error::InvalidLongOption(option) => write!(f, "Invalid option: --{}", option),
            Error::InvalidShortOption(option) => write!(f, "Invalid option: -{}", option),
            Error::Others(error) => error.fmt(f),
        }
    }
}
