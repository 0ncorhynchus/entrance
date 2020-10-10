use thiserror::Error;

#[derive(Error, Debug)]
pub enum EntranceError {
    #[error("Invalid number of arguments")]
    InvalidNumberOfArguments,
    #[error("Invalid option")]
    InvalidOption,
    #[error("Failed to parse")]
    ParseError(#[source] Box<dyn std::error::Error>),
}
