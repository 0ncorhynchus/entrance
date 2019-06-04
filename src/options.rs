use std::error;
use std::fmt;

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

#[derive(Debug, PartialEq, Eq)]
pub enum OptionItem {
    Long(String),
    Short(char),
}

/// A trait for parsing and containing options.
///
/// # Example
/// ```
/// use entrance::Options;
///
/// #[derive(Options)]
/// struct Opts {
///     #[description = "Print help message"]
///     #[short = 'h']
///     help: bool,
///
///     #[description = "Print version infomation"]
///     version: bool,
/// }
/// ```
///
/// # Limitation
/// The derive macro for `Options` supports only a struct with named fields.
/// Additionally, only `bool` is supported for the type of these fields.
pub trait Options: Sized {
    fn parse<I: Iterator<Item = OptionItem>>(options: I) -> Result<Self>;

    /// This associated function is for `HelpDisplay`.
    fn spec() -> &'static [Opt];
}

impl Options for () {
    fn parse<I: Iterator<Item = OptionItem>>(mut options: I) -> Result<Self> {
        if let Some(option) = options.next() {
            match option {
                OptionItem::Long(option) => {
                    return Err(OptionError::InvalidLongOption(option));
                }
                OptionItem::Short(o) => {
                    return Err(OptionError::InvalidShortOption(o));
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
    fn parse() -> Result<()> {
        let options = vec![
            OptionItem::Long("flag1".to_string()),
            OptionItem::Short('2'),
        ];
        let opts = <()>::parse(options.into_iter());
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
