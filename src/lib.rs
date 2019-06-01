mod arguments;
mod command;
mod options;

pub use crate::arguments::*;
pub use crate::command::*;
pub use crate::options::*;
pub use entrance_derive::*;
use std::error;

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;
