use std::error;
use std::fmt;
use std::iter::Peekable;

type Result<T> = std::result::Result<T, OptionError>;

#[derive(Debug, PartialEq)]
pub enum OptionError {
    InvalidShortOption(char),
    InvalidLongOption(String),
}

impl fmt::Display for OptionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OptionError::InvalidShortOption(flag) => write!(f, "Invalid option: -{}", flag),
            OptionError::InvalidLongOption(flag) => write!(f, "Invalid option: --{}", flag),
        }
    }
}

impl error::Error for OptionError {
    fn description(&self) -> &str {
        match self {
            OptionError::InvalidShortOption(_) | OptionError::InvalidLongOption(_) => {
                "Invalid option"
            }
        }
    }
}

/// A trait for parsing and containing options.
///
/// # Example
/// ```
/// use entrance::Options;
///
/// #[derive(Options)]
/// struct Opts {
///     help: bool,
///     version: bool,
/// }
/// ```
pub trait Options: Sized {
    fn consume<I: Iterator<Item = String>>(args: &mut Peekable<I>) -> Result<Self>;
    fn spec() -> &'static [Opt];
}

impl Options for () {
    fn consume<I: Iterator<Item = String>>(args: &mut Peekable<I>) -> Result<Self> {
        if let Some(arg) = args.peek() {
            if arg.starts_with("--") {
                return Err(OptionError::InvalidLongOption(arg[2..].to_string()));
            } else if arg.starts_with("-") {
                if let Some(f) = arg.chars().nth(1) {
                    return Err(OptionError::InvalidShortOption(f));
                } else {
                    args.next();
                }
            }
        }
        Ok(())
    }

    fn spec() -> &'static [Opt] {
        &[]
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Opt {
    pub long: &'static str,
    pub short: Option<char>,
    pub description: &'static str,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn consume() -> Result<()> {
        let args = ["--flag1", "-2", "arg1", "arg2"];
        let mut peekable = args.iter().map(|s| s.to_string()).peekable();
        let opts = <()>::consume(&mut peekable);
        assert_eq!(
            opts,
            Err(OptionError::InvalidLongOption("flag1".to_string()))
        );
        Ok(())
    }

    #[test]
    fn spec() {
        assert_eq!(<()>::spec().len(), 0);
    }
}
