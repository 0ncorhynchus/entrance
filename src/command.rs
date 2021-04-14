use crate::Result;
use crate::{Arguments, OptionItem, Options};
use std::iter::Peekable;
use std::marker::PhantomData;

pub trait Command: Sized {
    type Opts: Options;
    type Args: Arguments;

    fn exec(info: &CommandInfo<Self>, opts: Vec<Self::Opts>, args: Self::Args);

    fn build(name: &str, version: &str) -> CommandParser<Self> {
        CommandParser {
            info: CommandInfo::new(name, version),
        }
    }
}

#[derive(Clone, Debug)]
pub struct CommandInfo<C> {
    pub name: String,
    pub version: String,
    _phantom: PhantomData<C>,
}

impl<C> CommandInfo<C> {
    pub fn new(name: &str, version: &str) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
            _phantom: PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct CommandParser<C> {
    info: CommandInfo<C>,
}

impl<C> CommandParser<C> {
    pub fn new(name: &str, version: &str) -> Self {
        Self {
            info: CommandInfo::new(name, version),
        }
    }

    pub fn parse<I: Iterator<Item = String>>(self, args: I) -> Result<Call<C>>
    where
        C: Command,
    {
        // Skip the first element (= program_name)
        let mut args = args.skip(1).peekable();
        let options = take_options(&mut args);

        let mut opts = Vec::new();
        for opt in options {
            let opt = C::Opts::parse(opt);
            if let Ok(opt) = opt {
                if opt.is_informative() {
                    return Ok(Call::new(self.info, CallKind::Informative(opt)));
                }
                opts.push(Ok(opt));
            } else {
                opts.push(opt);
            }
        }

        let opts: Result<Vec<_>> = opts.into_iter().collect();
        Ok(Call::new(
            self.info,
            CallKind::Call((opts?, C::Args::parse(&mut args)?)),
        ))
    }
}

pub enum CallKind<C: Command> {
    Informative(C::Opts),
    Call((Vec<C::Opts>, C::Args)),
}

pub struct Call<C: Command> {
    info: CommandInfo<C>,
    kind: CallKind<C>,
}

impl<C: Command> Call<C> {
    pub fn new(info: CommandInfo<C>, kind: CallKind<C>) -> Self {
        Self { info, kind }
    }

    pub fn exec(self) {
        match self.kind {
            CallKind::Informative(opt) => opt.trigger_informative(&self.info),
            CallKind::Call((opts, args)) => C::exec(&self.info, opts, args),
        }
    }
}

fn take_options<I: Iterator<Item = String>>(args: &mut Peekable<I>) -> Vec<OptionItem> {
    let mut options = Vec::new();
    while let Some(arg) = args.peek() {
        if let Some(opt) = arg.strip_prefix("--") {
            if opt.is_empty() {
                break;
            }
            options.push(OptionItem::Long(opt.to_string()));
        } else if let Some(opts) = arg.strip_prefix("-") {
            if opts.is_empty() {
                break;
            }
            for c in opts.chars() {
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
pub struct HelpDisplay<'a, C>(&'a CommandInfo<C>);

impl<'a, C> HelpDisplay<'a, C> {
    pub fn new(info: &'a CommandInfo<C>) -> Self {
        Self(info)
    }
}

impl<'a, C: Command> std::fmt::Display for HelpDisplay<'a, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        const SPACER: &str = "    ";

        writeln!(f, "USAGE:")?;
        write!(f, "{indent}{}", self.0.name, indent = SPACER)?;
        if !C::Opts::spec().is_empty() {
            write!(f, " [OPTIONS]")?;
        }
        for arg in C::Args::spec() {
            write!(f, " <{}>", arg.name)?;
        }
        if let Some(args) = C::Args::var_spec() {
            write!(f, " [{}]...", args.name)?;
        }
        writeln!(f)?;

        format_options(f, SPACER, C::Opts::spec())?;

        let var_args_spec = C::Args::var_spec();
        if let Some(longest_length) = C::Args::spec()
            .iter()
            .chain(&var_args_spec)
            .map(|arg| arg.name.len())
            .max()
        {
            writeln!(f)?;
            writeln!(f, "ARGS:")?;
            for arg in C::Args::spec().iter().chain(&var_args_spec) {
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
        writeln!(f, "OPTIONS:")?;
        if opts.iter().any(|opt| opt.short.is_some()) {
            for opt in opts {
                writeln!(
                    f,
                    "{spacer}{} --{:<width$}{spacer}{}",
                    opt.short
                        .map(|f| format!("-{},", f))
                        .unwrap_or_else(|| "   ".to_string()),
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
    use crate::{parse_argument, Arg};
    use std::path::PathBuf;

    struct Args {
        arg1: String,
        arg2: i32,
        arg3: PathBuf,
    }

    impl Arguments for Args {
        fn parse<I: Iterator<Item = String>>(args: &mut I) -> Result<Self> {
            Ok(Self {
                arg1: parse_argument(args.next().unwrap())?,
                arg2: parse_argument(args.next().unwrap())?,
                arg3: parse_argument(args.next().unwrap())?,
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

        fn var_spec() -> Option<Arg> {
            None
        }
    }

    #[test]
    fn command() -> Result<()> {
        let args = ["sample", "arg1", "123", "path/to/file"];
        struct MyCommand;
        impl Command for MyCommand {
            type Opts = ();
            type Args = Args;

            fn exec(_info: &CommandInfo<Self>, _opts: Vec<()>, args: Args) {
                assert_eq!(args.arg1, "arg1".to_string());
                assert_eq!(args.arg2, 123);
                assert_eq!(args.arg3, "path/to/file".parse::<PathBuf>().unwrap());
            }
        }

        MyCommand::build("sample", "1.0.0")
            .parse(args.iter().map(|s| s.to_string()))?
            .exec();

        Ok(())
    }

    #[test]
    fn format_usage() {
        struct MyCommand;
        impl Command for MyCommand {
            type Opts = ();
            type Args = Args;

            fn exec(_info: &CommandInfo<Self>, _opts: Vec<()>, _args: Args) {}
        }

        let info = CommandInfo::<MyCommand>::new("sample", "1.0.0");
        let usage = HelpDisplay::new(&info);
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
