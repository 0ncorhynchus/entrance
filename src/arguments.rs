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

/// A trait for parsing and containing arguments.
///
/// # Example
/// ```
/// use entrance::Arguments;
/// use std::path::PathBuf;
///
/// #[derive(Arguments)]
/// struct Args {
///     #[description = "The number of lines"]
///     num: f64,
///
///     #[description = "Path to a file"]
///     file: PathBuf,
/// }
/// ```
pub trait Arguments: Sized {
    fn parse<I: Iterator<Item = String>>(args: I) -> Result<Self>;

    /// This associated function is for `HelpDisplay`.
    fn spec() -> &'static [Arg];
}

impl Arguments for () {
    fn parse<I: Iterator<Item = String>>(_args: I) -> Result<Self> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() -> Result<()> {
        let args = ["arg1", "123", "path/to/file"];
        let mut args = args.iter().map(|s| s.to_string());
        let _ = <()>::parse(&mut args)?;

        assert_eq!(args.next(), Some("arg1".to_string()));

        Ok(())
    }

    #[test]
    fn spec() {
        assert_eq!(<()>::spec().len(), 0);
    }
}
