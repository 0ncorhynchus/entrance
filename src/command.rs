use crate::Result;
use crate::{Arguments, OptionItem, Options};
use std::iter::Peekable;
use std::marker::PhantomData;

#[derive(Debug, PartialEq)]
pub enum CallType<Opts, Args> {
    Informative(Opts),
    Normal(Vec<Opts>, Args),
}

/// Helper struct for parsing command line arguments and returning `CallType`.
#[derive(Debug)]
pub struct Command<Opts, Args> {
    name: String,
    _phantom: PhantomData<CallType<Opts, Args>>,
}

impl<Opts, Args> Command<Opts, Args>
where
    Opts: Options,
    Args: Arguments,
{
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            _phantom: PhantomData,
        }
    }

    pub fn parse<I: Iterator<Item = String>>(&self, args: I) -> Result<CallType<Opts, Args>> {
        // Skip the first element (= program_name)
        let mut args = args.skip(1).peekable();
        let options = take_options(&mut args);

        let mut opts: Vec<_> = options.into_iter().map(Opts::parse).collect();
        let informative_idx = opts.iter().enumerate().find_map(|(idx, opt)| {
            if let Ok(opt) = opt {
                if opt.is_informative() {
                    return Some(idx);
                }
            }
            None
        });
        if let Some(idx) = informative_idx {
            return Ok(CallType::Informative(opts.remove(idx).unwrap()));
        }

        let opts: Result<Vec<_>> = opts.into_iter().collect();
        Ok(CallType::Normal(opts?, Args::parse(&mut args)?))
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn help(&self) {
        let help_message: HelpDisplay<Opts, Args> = HelpDisplay::new(self.name());
        println!("{}", help_message);
    }

    pub fn version(&self) {
        println!("{} 1.0.0", self.name());
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
pub struct HelpDisplay<'a, Opts, Args>(&'a str, PhantomData<(Opts, Args)>);

impl<'a, Opts, Args> HelpDisplay<'a, Opts, Args> {
    fn new(name: &'a str) -> Self {
        Self(name, PhantomData)
    }
}

impl<'a, Opts, Args> std::fmt::Display for HelpDisplay<'a, Opts, Args>
where
    Opts: Options,
    Args: Arguments,
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
        if let Some(args) = Args::var_spec() {
            write!(f, " [{}]...", args.name)?;
        }
        writeln!(f)?;

        format_options(f, SPACER, Opts::spec())?;

        let var_args_spec = Args::var_spec();
        if let Some(longest_length) = Args::spec()
            .iter()
            .chain(&var_args_spec)
            .map(|arg| arg.name.len())
            .max()
        {
            writeln!(f)?;
            writeln!(f, "ARGS:")?;
            for arg in Args::spec().iter().chain(&var_args_spec) {
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
        let command: Command<(), Args> = Command::new("sample");

        let args = match command.parse(args.iter().map(|s| s.to_string()))? {
            CallType::Normal(_opts, args) => args,
            _ => panic!("CallType::Normal variant is expected for `command.call_type()`."),
        };

        assert_eq!(args.arg1, "arg1".to_string());
        assert_eq!(args.arg2, 123);
        assert_eq!(args.arg3, "path/to/file".parse::<PathBuf>().unwrap());

        Ok(())
    }

    #[test]
    fn format_usage() {
        let usage = HelpDisplay::<(), Args>::new("sample");
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
