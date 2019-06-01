use crate::Result;
use std::error;
use std::fmt;

#[derive(Debug)]
pub enum ArgumentError {
    InvalidNumberOfArguments,
}

impl fmt::Display for ArgumentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ArgumentError::InvalidNumberOfArguments => write!(f, "Invalid number of arguments"),
        }
    }
}

impl error::Error for ArgumentError {
    fn description(&self) -> &str {
        match self {
            ArgumentError::InvalidNumberOfArguments => "Invalid number of arguments",
        }
    }
}

pub trait Arguments: Sized {
    fn parse_from<I: Iterator<Item = String>>(args: I) -> Result<Self>;
    fn spec() -> &'static [Arg];
}

impl Arguments for () {
    fn parse_from<I: Iterator<Item = String>>(_args: I) -> Result<Self> {
        Ok(())
    }

    fn spec() -> &'static [Arg] {
        &[]
    }
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

    struct Args {
        arg1: String,
        arg2: i32,
        arg3: PathBuf,
    }

    impl Arguments for Args {
        fn parse_from<I: Iterator<Item = String>>(mut args: I) -> Result<Self> {
            Ok(Self {
                arg1: args.next().unwrap().parse()?,
                arg2: args.next().unwrap().parse()?,
                arg3: args.next().unwrap().parse()?,
            })
        }

        fn spec() -> &'static [Arg] {
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
        let result = Args::parse_from(args.iter().map(|s| s.to_string()))?;

        assert_eq!(result.arg1, "arg1".to_string());
        assert_eq!(result.arg2, 123);
        assert_eq!(result.arg3, "path/to/file".parse::<PathBuf>().unwrap());

        Ok(())
    }
}
