use crate::{ErrorKind, Result};

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
/// use entrance::Options;
///
/// #[derive(Options)]
/// enum Opts {
///     #[description = "Print help message"]
///     #[short = 'h']
///     Help,
///
///     #[description = "Print version infomation"]
///     Version,
/// }
/// ```
///
/// # Limitation
/// The derive macro for `Options` supports only an Enum whose variants don't have any field.
pub trait Options: Sized {
    fn parse<I: Iterator<Item = OptionItem>>(options: I) -> Result<Vec<Self>>;

    /// This associated function is for `HelpDisplay`.
    fn spec() -> &'static [Opt];
}

impl Options for () {
    fn parse<I: Iterator<Item = OptionItem>>(mut options: I) -> Result<Vec<Self>> {
        if options.next().is_some() {
            return Err(ErrorKind::InvalidOption.into());
        }
        Ok(Vec::new())
    }

    fn spec() -> &'static [Opt] {
        &[]
    }
}

pub trait InformativeOption: Sized {
    fn parse<'a, I: Iterator<Item = &'a OptionItem>>(options: I) -> Option<Self>;

    /// This associated function is for `HelpDisplay`.
    fn spec() -> &'static [Opt];
}

impl InformativeOption for () {
    fn parse<'a, I: Iterator<Item = &'a OptionItem>>(_options: I) -> Option<Self> {
        None
    }

    fn spec() -> &'static [Opt] {
        &[]
    }
}

pub enum DefaultInformativeOption {
    Help,
    Version,
}

impl InformativeOption for DefaultInformativeOption {
    fn parse<'a, I: Iterator<Item = &'a OptionItem>>(options: I) -> Option<Self> {
        for opt in options {
            match opt {
                OptionItem::Long(opt) => match opt.as_str() {
                    "help" => {
                        return Some(DefaultInformativeOption::Help);
                    }
                    "version" => {
                        return Some(DefaultInformativeOption::Version);
                    }
                    _ => {}
                },
                OptionItem::Short(opt) => match opt {
                    'h' => {
                        return Some(DefaultInformativeOption::Help);
                    }
                    'V' => {
                        return Some(DefaultInformativeOption::Version);
                    }
                    _ => {}
                },
            }
        }
        None
    }

    /// This associated function is for `HelpDisplay`.
    fn spec() -> &'static [Opt] {
        static OPTS: [Opt; 2] = [
            Opt {
                long: "help",
                short: Some('h'),
                description: "Print help message",
            },
            Opt {
                long: "version",
                short: Some('V'),
                description: "Print the version",
            },
        ];
        &OPTS
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
        let opts = <() as Options>::parse(options.into_iter());
        match opts {
            Ok(_) => {
                panic!("Err variant is expected.");
            }
            Err(error) => match error.kind() {
                ErrorKind::InvalidOption => {}
                _ => {
                    panic!("ErrorKind::InvalidOption is expected.");
                }
            },
        }

        Ok(())
    }

    #[test]
    fn spec() {
        assert_eq!(<() as Options>::spec().len(), 0);
    }
}
