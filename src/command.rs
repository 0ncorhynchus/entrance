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

    pub fn help(&self) -> HelpDisplay<A> {
        HelpDisplay::new(&self.name)
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

#[derive(Debug)]
pub struct HelpDisplay<'a, A>(&'a str, PhantomData<A>);

impl<'a, A> HelpDisplay<'a, A> {
    fn new(name: &'a str) -> Self {
        Self(name, PhantomData)
    }
}

impl<'a, A> std::fmt::Display for HelpDisplay<'a, A>
where
    A: Args,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        const SPACER: &'static str = "    ";

        writeln!(f, "USAGE:")?;
        write!(f, "{indent}{}", self.0, indent = SPACER)?;
        for arg in A::args() {
            write!(f, " <{}>", arg.name)?;
        }
        writeln!(f, "")?;

        writeln!(f, "")?;

        if let Some(longest_length) = A::args().iter().map(|arg| arg.name.len()).max() {
            writeln!(f, "ARGS:")?;
            for arg in A::args() {
                writeln!(
                    f,
                    "{spacer}{:<width$}{spacer}{}",
                    arg.name,
                    arg.description,
                    spacer = SPACER,
                    width = longest_length
                )?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Arg;
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

        fn args() -> &'static [Arg] {
            const ARGS: [Arg; 3] = [
                Arg::new("arg1", "This is parsed as String"),
                Arg::new("arg2", "This is parsed as i32"),
                Arg::new("arg3", "This is parsed as PathBuf"),
            ];
            &ARGS
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

    #[test]
    fn format_usage() {
        let usage = HelpDisplay::<Arguments>::new("sample");
        assert_eq!(
            usage.to_string(),
            "\
USAGE:
    sample <arg1> <arg2> <arg3>

ARGS:
    arg1    This is parsed as String
    arg2    This is parsed as i32
    arg3    This is parsed as PathBuf
"
            .to_string()
        );
    }
}
