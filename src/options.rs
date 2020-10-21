use crate::{Arguments, Command, Error, Result};

#[derive(Debug, PartialEq, Eq)]
#[doc(hidden)]
pub enum OptionItem {
    Long(String),
    Short(char),
}

/// A trait for parsing and containing options.
///
/// # Example
/// ```
/// use entrance::{Command, Options};
///
/// #[derive(Options)]
/// enum Opts {
///     #[entrance(description = "Print help message")]
///     #[entrance(short = 'h')]
///     #[entrance(informative(Command::help))]
///     Help,
///
///     #[entrance(description = "Print version infomation")]
///     #[entrance(informative(Command::version))]
///     Version,
/// }
/// ```
///
/// # Limitation
/// The derive macro for `Options` supports only an Enum whose variants don't have any field.
pub trait Options: Sized {
    fn parse(option: OptionItem) -> Result<Self>;

    fn is_informative(&self) -> bool;

    fn trigger_informative<Args: Arguments>(&self, command: &Command<Self, Args>);

    /// This associated function is for `HelpDisplay`.
    fn spec() -> &'static [Opt];
}

impl Options for () {
    fn parse(_: OptionItem) -> Result<Self> {
        Err(Error::InvalidOption)
    }

    fn is_informative(&self) -> bool {
        unimplemented!()
    }

    fn trigger_informative<Args: Arguments>(&self, _: &Command<Self, Args>) {
        unimplemented!()
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

        for option in options {
            let option = <() as Options>::parse(option);
            assert!(option.is_err());
            let is_invalid_option = match option.unwrap_err() {
                Error::InvalidOption => true,
                _ => false,
            };
            assert!(is_invalid_option);
        }

        Ok(())
    }

    #[test]
    fn spec() {
        assert_eq!(<() as Options>::spec().len(), 0);
    }
}
