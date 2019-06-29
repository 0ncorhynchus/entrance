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
//! struct Opts {
//!     #[description = "Print help message"]
//!     #[short = 'h']
//!     help: bool,
//!
//!     #[description = "Use verbose output"]
//!     #[short = 'v']
//!     verbose: bool,
//!
//!     #[description = "Print version information"]
//!     version: bool,
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
//! let command = Command::<Opts, Args>::new("program").parse_options(args).unwrap();
//!
//! if command.options().version {
//!     // Print version information and exit.
//! }
//!
//! if command.options().help {
//!     println!("{}", command.help());
//!     // Exit
//! }
//!
//! // Parse the other arguments
//! let command = command.parse_arguments().unwrap();
//!
//! assert!(!command.options().help);
//! assert!(!command.options().version);
//! assert!(command.options().verbose);
//!
//! assert_eq!(command.arguments().path, PathBuf::from("path/to/file"));
//! ```

mod arguments;
mod command;
mod error;
mod options;

pub use crate::arguments::*;
pub use crate::command::*;
pub use crate::error::*;
pub use crate::options::*;
pub use entrance_derive::*;

use failure::{Fail, ResultExt};

/// A helper function to parse argument
pub fn parse_argument<T, E>(arg: String) -> Result<T>
where
    T: std::str::FromStr<Err = E>,
    E: Fail,
{
    // The below code can't be compiled because of failure in type inference
    //
    // ```rust
    // Ok(arg.parse().context(ErrorKind::ParseError)?)
    // ```
    let result: std::result::Result<T, E> = arg.parse();
    Ok(result.context(ErrorKind::ParseError)?)
}

pub fn parse_variable_argument<T, E, I, V>(args: I) -> Result<V>
where
    T: std::str::FromStr<Err = E>,
    E: Fail,
    I: Iterator<Item = String>,
    V: std::iter::FromIterator<T>,
{
    args.map(|arg| {
        let result: std::result::Result<T, E> = arg.parse();
        Ok(result.context(ErrorKind::ParseError)?)
    })
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
