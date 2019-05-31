pub use entrance_derive::Args;
use std::error;
use std::fmt;
use std::marker::PhantomData;

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub enum Error {
    InvalidNumberOfArguments,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InvalidNumberOfArguments => write!(f, "Invalid number of arguments"),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self {
            Error::InvalidNumberOfArguments => "Invalid number of arguments",
        }
    }
}

pub trait Args: Sized {
    fn parse_from<I: Iterator<Item = String>>(args: I) -> Result<Self>;
}

#[derive(Debug)]
pub struct Command<Arguments> {
    name: String,
    args: Arguments,
}

impl<A> Command<A> {
    pub fn new(name: &str) -> CommandPrecursor<Self> {
        CommandPrecursor {
            name: name.to_string(),
            _phantom: PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct CommandPrecursor<Command> {
    name: String,
    _phantom: PhantomData<Command>,
}

impl<A> CommandPrecursor<Command<A>>
where
    A: Args,
{
    pub fn parse_args<I: Iterator<Item = String>>(self, mut args: I) -> Result<Command<A>> {
        let _program_name = args.next();
        Ok(Command {
            name: self.name,
            args: A::parse_from(args)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    struct Arguments {
        arg1: String,
        arg2: i32,
        arg3: PathBuf,
    }

    impl Args for Arguments {
        fn parse_from<I: Iterator<Item = String>>(mut args: I) -> Result<Self> {
            Ok(Self {
                arg1: args.next().unwrap().parse()?,
                arg2: args.next().unwrap().parse()?,
                arg3: args.next().unwrap().parse()?,
            })
        }
    }

    #[test]
    fn parse_from() -> Result<()> {
        let args = ["arg1", "123", "path/to/file"];
        let result = Arguments::parse_from(args.iter().map(|s| s.to_string()))?;

        assert_eq!(result.arg1, "arg1".to_string());
        assert_eq!(result.arg2, 123);
        assert_eq!(result.arg3, "path/to/file".parse::<PathBuf>().unwrap());

        Ok(())
    }

    #[test]
    fn command() -> Result<()> {
        let args = ["sample", "arg1", "123", "path/to/file"];
        let command: Command<Arguments> =
            Command::new("sample").parse_args(args.into_iter().map(|s| s.to_string()))?;

        assert_eq!(command.args.arg1, "arg1".to_string());
        assert_eq!(command.args.arg2, 123);
        assert_eq!(
            command.args.arg3,
            "path/to/file".parse::<PathBuf>().unwrap()
        );

        Ok(())
    }
}
