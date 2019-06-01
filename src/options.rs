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

pub trait Options: Sized {
    fn consume<I: Iterator<Item = String>>(args: &mut Peekable<I>) -> Result<Self>;
    fn spec() -> &'static [Opt];
}

impl Options for () {
    fn consume<I: Iterator<Item = String>>(_args: &mut Peekable<I>) -> Result<Self> {
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

    #[derive(Debug, PartialEq)]
    struct Opts {
        flag1: bool,
        flag2: bool,
        flag3: bool,
    }

    impl Options for Opts {
        fn consume<I: Iterator<Item = String>>(args: &mut Peekable<I>) -> Result<Self> {
            let mut flag1 = false;
            let mut flag2 = false;
            let mut flag3 = false;

            while let Some(arg) = args.peek() {
                if arg.starts_with("--") {
                    match &arg[2..] {
                        "flag1" => flag1 = true,
                        "flag2" => flag2 = true,
                        "flag3" => flag3 = true,
                        flag => {
                            return Err(OptionError::InvalidLongOption(flag.to_string()));
                        }
                    }
                } else if arg.starts_with("-") {
                    for c in arg[1..].chars() {
                        match c {
                            '1' => flag1 = true,
                            '2' => flag2 = true,
                            '3' => flag3 = true,
                            flag => {
                                return Err(OptionError::InvalidShortOption(flag));
                            }
                        }
                    }
                } else {
                    break;
                }
                args.next(); // Consume an element
            }

            Ok(Self {
                flag1,
                flag2,
                flag3,
            })
        }

        fn spec() -> &'static [Opt] {
            static OPTS: [Opt; 3] = [
                Opt {
                    long: "flag1",
                    short: Some('1'),
                    description: "Option 1",
                },
                Opt {
                    long: "flag2",
                    short: Some('2'),
                    description: "Option 2",
                },
                Opt {
                    long: "flag3",
                    short: Some('3'),
                    description: "Option 3",
                },
            ];
            &OPTS
        }
    }

    #[test]
    fn consume() -> Result<()> {
        let args = ["--flag1", "-2", "arg1", "arg2"];
        let mut peekable = args.iter().map(|s| s.to_string()).peekable();
        let opts = Opts::consume(&mut peekable)?;
        assert!(opts.flag1);
        assert!(opts.flag2);
        assert!(!opts.flag3);
        assert_eq!(peekable.next(), Some("arg1".to_string()));
        Ok(())
    }

    #[test]
    fn fail_to_consume_long_option() {
        let args = ["--flag1", "-2", "--unknown", "arg1", "arg2"];
        let mut peekable = args.iter().map(|s| s.to_string()).peekable();
        let ops = Opts::consume(&mut peekable);
        assert_eq!(
            ops,
            Err(OptionError::InvalidLongOption("unknown".to_string()))
        );
        assert_eq!(peekable.next(), Some("--unknown".to_string()));
    }

    #[test]
    fn fail_to_consume_short_option() {
        let args = ["--flag1", "-2", "-x", "arg1", "arg2"];
        let mut peekable = args.iter().map(|s| s.to_string()).peekable();
        let ops = Opts::consume(&mut peekable);
        assert_eq!(ops, Err(OptionError::InvalidShortOption('x')));
        assert_eq!(peekable.next(), Some("-x".to_string()));
    }
}
