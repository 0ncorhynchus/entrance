use std::error;
use std::fmt;
use std::iter::Peekable;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidShortOption(char),
    InvalidLongOption(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InvalidShortOption(flag) => write!(f, "Invalid option: -{}", flag),
            Error::InvalidLongOption(flag) => write!(f, "Invalid option: --{}", flag),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self {
            Error::InvalidShortOption(_) | Error::InvalidLongOption(_) => "Invalid option",
        }
    }
}

pub trait Options: Sized {
    fn consume<I: Iterator<Item = String>>(args: &mut Peekable<I>) -> Result<Self>;
}

impl Options for () {
    fn consume<I: Iterator<Item = String>>(_args: &mut Peekable<I>) -> Result<Self> {
        Ok(())
    }
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
                            return Err(Error::InvalidLongOption(flag.to_string()));
                        }
                    }
                } else if arg.starts_with("-") {
                    for c in arg[1..].chars() {
                        match c {
                            '1' => flag1 = true,
                            '2' => flag2 = true,
                            '3' => flag3 = true,
                            flag => {
                                return Err(Error::InvalidShortOption(flag));
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
        assert_eq!(ops, Err(Error::InvalidLongOption("unknown".to_string())));
        assert_eq!(peekable.next(), Some("--unknown".to_string()));
    }

    #[test]
    fn fail_to_consume_short_option() {
        let args = ["--flag1", "-2", "-x", "arg1", "arg2"];
        let mut peekable = args.iter().map(|s| s.to_string()).peekable();
        let ops = Opts::consume(&mut peekable);
        assert_eq!(ops, Err(Error::InvalidShortOption('x')));
        assert_eq!(peekable.next(), Some("-x".to_string()));
    }
}
