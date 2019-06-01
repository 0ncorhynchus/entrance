//! Utilities for parsing command line arguments
//!
//! `entrance` provides the type assisted tools to parse command line argumuments.
//!
//! # Usage
//!
//! An example of `Command` with `Options` are:
//!
//! ```
//! use entrance::*;
//!
//! #[derive(Options)]
//! struct Opts {
//!     help: bool,
//!     verbose: bool,
//!     version: bool,
//! }
//!
//! let args = ["program", "--version"].iter().map(|s| s.to_string());
//! let command: Command<Opts, ()> =
//!     Command::new("program").parse_args(args).unwrap();
//!
//! assert!(command.options().version);
//! assert!(!command.options().help);
//! assert!(!command.options().verbose);
//! ```

mod arguments;
mod command;
mod options;

pub use crate::arguments::*;
pub use crate::command::*;
pub use crate::options::*;
pub use entrance_derive::*;
use std::error;

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;
