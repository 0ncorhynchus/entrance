use crate::Result;
use crate::{Arguments, OptionItem, Options, VariableArgument};
use std::iter::Peekable;
use std::marker::PhantomData;

/// A struct containing parsed options and arguments.
#[derive(Debug)]
pub struct Command<Opts, Args, VarArg> {
    name: String,
    options: Opts,
    args: Args,
    var_arg: VarArg,
}

impl<Opts, Args, VarArg> Command<Opts, Args, VarArg> {
    pub fn new(name: &str) -> CommandPrecursor<Self> {
        CommandPrecursor {
            name: name.to_string(),
            _phantom: PhantomData,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn options(&self) -> &Opts {
        &self.options
    }

    pub fn arguments(&self) -> &Args {
        &self.args
    }

    pub fn variable_argument(&self) -> &VarArg {
        &self.var_arg
    }

    pub fn help(&self) -> HelpDisplay<Opts, Args, VarArg> {
        HelpDisplay::new(&self.name)
    }
}

/// Helper struct for parsing command line arguments and returning `Command`.
#[derive(Debug)]
pub struct CommandPrecursor<Command> {
    name: String,
    _phantom: PhantomData<Command>,
}

impl<Opts, Args, VarArg> CommandPrecursor<Command<Opts, Args, VarArg>>
where
    Opts: Options,
    Args: Arguments,
    VarArg: VariableArgument,
{
    pub fn parse<I: Iterator<Item = String>>(self, args: I) -> Result<Command<Opts, Args, VarArg>> {
        Ok(self.parse_options(args)?.parse_arguments()?)
    }

    pub fn parse_options<I: Iterator<Item = String>>(
        self,
        args: I,
    ) -> Result<OptionParsedCommand<Peekable<I>, Opts, Args, VarArg>> {
        let mut args = args.peekable();
        let _program_name = args.next();
        Ok(OptionParsedCommand {
            name: self.name,
            options: Opts::parse(take_options(&mut args).into_iter())?,
            iter: args,
            _phantom: PhantomData,
        })
    }
}

/// A struct as an intermediate just after parsing options.
///
/// This `struct` is created by `parse_option` method on `CommandPrecursor`.
#[derive(Debug)]
pub struct OptionParsedCommand<I, Opts, Args, VarArg> {
    name: String,
    iter: I,
    options: Opts,
    _phantom: PhantomData<(Args, VarArg)>,
}

impl<I, Opts, Args, VarArg> OptionParsedCommand<I, Opts, Args, VarArg> {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn options(&self) -> &Opts {
        &self.options
    }

    pub fn help(&self) -> HelpDisplay<Opts, Args, VarArg> {
        HelpDisplay::new(&self.name)
    }

    /// parse the other arguments.
    pub fn parse_arguments(self) -> Result<Command<Opts, Args, VarArg>>
    where
        I: Iterator<Item = String>,
        Args: Arguments,
        VarArg: VariableArgument,
    {
        let mut iter = self.iter;
        Ok(Command {
            name: self.name,
            options: self.options,
            args: Args::parse(&mut iter)?,
            var_arg: VarArg::parse(&mut iter)?,
        })
    }
}

fn take_options<I: Iterator<Item = String>>(args: &mut Peekable<I>) -> Vec<OptionItem> {
    let mut options = Vec::new();
    while let Some(arg) = args.peek() {
        if arg.starts_with("--") {
            if arg.len() == 2 {
                break;
            }
            options.push(OptionItem::Long(arg[2..].to_string()));
        } else if arg.starts_with('-') {
            if arg.len() == 1 {
                break;
            }
            for c in arg[1..].chars() {
                options.push(OptionItem::Short(c));
            }
        } else {
            break;
        }
        args.next(); // Consume an argument
    }
    options
}

/// Helper struct for printing help messages with `format!` and `{}`.
#[derive(Debug)]
pub struct HelpDisplay<'a, Opts, Args, VarArg>(&'a str, PhantomData<(Opts, Args, VarArg)>);

impl<'a, Opts, Args, VarArg> HelpDisplay<'a, Opts, Args, VarArg> {
    fn new(name: &'a str) -> Self {
        Self(name, PhantomData)
    }
}

impl<'a, Opts, Args, VarArg> std::fmt::Display for HelpDisplay<'a, Opts, Args, VarArg>
where
    Opts: Options,
    Args: Arguments,
    VarArg: VariableArgument,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        const SPACER: &str = "    ";

        writeln!(f, "USAGE:")?;
        write!(f, "{indent}{}", self.0, indent = SPACER)?;
        if !Opts::spec().is_empty() {
            write!(f, " [OPTIONS]")?;
        }
        for arg in Args::spec() {
            write!(f, " <{}>", arg.name)?;
        }
        if let Some(args) = VarArg::spec() {
            write!(f, " [{}]...", args.name)?;
        }
        writeln!(f)?;

        format_options(f, SPACER, Opts::spec())?;

        let var_arg_spec = VarArg::spec();
        if let Some(longest_length) = Args::spec()
            .iter()
            .chain(&var_arg_spec)
            .map(|arg| arg.name.len())
            .max()
        {
            writeln!(f)?;
            writeln!(f, "ARGS:")?;
            for arg in Args::spec().iter().chain(&var_arg_spec) {
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

fn format_options(
    f: &mut std::fmt::Formatter,
    spacer: &str,
    opts: &[crate::Opt],
) -> std::fmt::Result {
    if let Some(longest_length) = opts.iter().map(|opt| opt.long.len()).max() {
        writeln!(f)?;
        writeln!(f, "OPTIONS")?;
        if opts.iter().any(|opt| opt.short.is_some()) {
            for opt in opts {
                writeln!(
                    f,
                    "{spacer}{}, --{:<width$}{spacer}{}",
                    opt.short
                        .map(|f| ['-', f].iter().collect())
                        .unwrap_or_else(|| "  ".to_string()),
                    opt.long,
                    opt.description,
                    spacer = spacer,
                    width = longest_length
                )?;
            }
        } else {
            for opt in opts {
                writeln!(
                    f,
                    "{spacer}--{:<width$}{spacer}{}",
                    opt.long,
                    opt.description,
                    spacer = spacer,
                    width = longest_length
                )?;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Arg;
    use std::path::PathBuf;

    struct Args {
        arg1: String,
        arg2: i32,
        arg3: PathBuf,
    }

    impl Arguments for Args {
        fn parse<I: Iterator<Item = String>>(args: &mut I) -> Result<Self> {
            Ok(Self {
                arg1: args.next().unwrap().parse()?,
                arg2: args.next().unwrap().parse()?,
                arg3: args.next().unwrap().parse()?,
            })
        }

        fn spec() -> &'static [Arg] {
            const ARGS: [Arg; 3] = [
                Arg {
                    name: "arg1",
                    description: "This is parsed as String",
                },
                Arg {
                    name: "arg2",
                    description: "This is parsed as i32",
                },
                Arg {
                    name: "arg3",
                    description: "This is parsed as PathBuf",
                },
            ];
            &ARGS
        }
    }

    #[test]
    fn command() -> Result<()> {
        let args = ["sample", "arg1", "123", "path/to/file"];
        let command: Command<(), Args, ()> =
            Command::new("sample").parse(args.iter().map(|s| s.to_string()))?;

        assert_eq!(command.arguments().arg1, "arg1".to_string());
        assert_eq!(command.arguments().arg2, 123);
        assert_eq!(
            command.arguments().arg3,
            "path/to/file".parse::<PathBuf>().unwrap()
        );

        Ok(())
    }

    #[test]
    fn format_usage() {
        let usage = HelpDisplay::<(), Args, ()>::new("sample");
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
