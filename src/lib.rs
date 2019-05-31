mod command;

pub use crate::command::*;
pub use entrance_derive::Args;
use std::error;
use std::fmt;

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub enum Error {
    InvalidNumberOfArguments,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InvalidNumberOfArguments => write!(f, "Invalid number of arguments"),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self {
            Error::InvalidNumberOfArguments => "Invalid number of arguments",
        }
    }
}

pub trait Args: Sized {
    fn parse_from<I: Iterator<Item = String>>(args: I) -> Result<Self>;
    fn args() -> &'static [Arg];
}

#[derive(Debug, Clone, Copy)]
pub struct Arg {
    pub name: &'static str,
    pub description: &'static str,
}

impl Arg {
    pub const fn new(name: &'static str, description: &'static str) -> Self {
        Self { name, description }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    struct Arguments {
        arg1: String,
        arg2: i32,
        arg3: PathBuf,
    }

    impl Args for Arguments {
        fn parse_from<I: Iterator<Item = String>>(mut args: I) -> Result<Self> {
            Ok(Self {
                arg1: args.next().unwrap().parse()?,
                arg2: args.next().unwrap().parse()?,
                arg3: args.next().unwrap().parse()?,
            })
        }

        fn args() -> &'static [Arg] {
            const ARGS: [Arg; 3] = [
                Arg::new("arg1", ""),
                Arg::new("arg2", ""),
                Arg::new("arg3", ""),
            ];
            &ARGS
        }
    }

    #[test]
    fn parse_from() -> Result<()> {
        let args = ["arg1", "123", "path/to/file"];
        let result = Arguments::parse_from(args.iter().map(|s| s.to_string()))?;

        assert_eq!(result.arg1, "arg1".to_string());
        assert_eq!(result.arg2, 123);
        assert_eq!(result.arg3, "path/to/file".parse::<PathBuf>().unwrap());

        Ok(())
    }
}
