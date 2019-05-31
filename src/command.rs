use crate::Args;
use crate::Result;
use std::marker::PhantomData;

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

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn args(&self) -> &A {
        &self.args
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
    fn command() -> Result<()> {
        let args = ["sample", "arg1", "123", "path/to/file"];
        let command: Command<Arguments> =
            Command::new("sample").parse_args(args.into_iter().map(|s| s.to_string()))?;

        assert_eq!(command.args().arg1, "arg1".to_string());
        assert_eq!(command.args().arg2, 123);
        assert_eq!(
            command.args().arg3,
            "path/to/file".parse::<PathBuf>().unwrap()
        );

        Ok(())
    }
}
