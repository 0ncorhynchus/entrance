use crate::{Error, Result};

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
        if options.next().is_some() {
            return Err(Error::InvalidOption);
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
        match opts {
            Ok(_) => {
                panic!("Err(InvalidOption) is expected.");
            }
            Err(error) => match error {
                Error::InvalidOption => {}
                _ => {
                    panic!("Err(InvalidOption) is expected.");
                }
            },
        }

        Ok(())
    }

    #[test]
    fn spec() {
        assert_eq!(<()>::spec().len(), 0);
    }
}
