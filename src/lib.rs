//! Utilities for parsing command line arguments
//!
//! `entrance` provides type assisted tools for parsing command line argumuments.
//!
//! # Usage
//!
//! ```
//! use entrance::*;
//! use std::path::PathBuf;
//!
//! #[derive(Options)]
//! enum Opts {
//!     #[description = "Print the help message"]
//!     #[short = 'h']
//!     #[informative]
//!     Help,
//!
//!     #[description = "Use verbose output"]
//!     #[short = 'v']
//!     Verbose,
//! }
//!
//! #[derive(Arguments)]
//! struct Args {
//!     #[description = "Path to a file"]
//!     path: PathBuf,
//! }
//!
//! let args = ["program", "-v", "path/to/file"].iter().map(|s| s.to_string());
//!
//! // Parse only options to exit immediately with "--version" or "--help".
//! let command = Command::<Opts, Args>::new("program");
//!
//! match command.parse(args).unwrap() {
//!     CallType::Informative(_) => {
//!         println!("{}", command.help());
//!     },
//!     CallType::Normal(opts, args) => {
//!         assert!(!opts.is_empty());
//!         assert_eq!(args.path, PathBuf::from("path/to/file"));
//!     }
//! }
//! ```

mod arguments;
mod command;
mod error;
mod options;

pub use crate::arguments::*;
pub use crate::command::*;
pub use crate::error::EntranceError as Error;
pub use crate::options::*;
pub use entrance_derive::*;

pub type Result<T> = std::result::Result<T, Error>;

/// A helper function to parse argument
pub fn parse_argument<T, E>(arg: String) -> Result<T>
where
    T: std::str::FromStr<Err = E>,
    E: std::error::Error + 'static,
{
    arg.parse().map_err(|err| Error::ParseError(Box::new(err)))
}

pub fn parse_variable_argument<T, E, I, V>(args: I) -> Result<V>
where
    T: std::str::FromStr<Err = E>,
    E: std::error::Error + 'static,
    I: Iterator<Item = String>,
    V: std::iter::FromIterator<T>,
{
    args.map(|arg| arg.parse().map_err(|err| Error::ParseError(Box::new(err))))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_argument() {
        let parsed: Result<f64> = parse_argument("1.0".to_string());
        assert!(parsed.is_ok());
        assert_eq!(parsed.unwrap(), 1.0);

        let parsed: Result<f64> = parse_argument("not float number".to_string());
        assert!(parsed.is_err());
        assert_eq!(parsed.unwrap_err().kind(), ErrorKind::ParseError);
    }

    #[test]
    fn test_parse_variable_argument() {
        let args = vec!["1.0", "2.0", "3.0"].into_iter().map(String::from);
        let parsed: Result<Vec<f64>> = parse_variable_argument(args);
        assert!(parsed.is_ok());
        assert_eq!(parsed.unwrap(), vec![1.0, 2.0, 3.0]);

        let args = vec!["1.0", "not float number", "3.0"]
            .into_iter()
            .map(String::from);
        let parsed: Result<Vec<f64>> = parse_variable_argument(args);
        assert!(parsed.is_err());
        assert_eq!(parsed.unwrap_err().kind(), ErrorKind::ParseError);
    }
}
