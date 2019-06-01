use entrance::{Args, Options};
use std::path::PathBuf;

#[test]
fn struct_with_named_fields() -> entrance::Result<()> {
    #[derive(Args)]
    struct Arguments {
        arg1: String,
        arg2: i32,
        arg3: PathBuf,
    }

    let args = ["arg1", "123", "path/to/file"];
    let result = Arguments::parse_from(args.iter().map(|s| s.to_string()))?;

    assert_eq!(result.arg1, "arg1".to_string());
    assert_eq!(result.arg2, 123);
    assert_eq!(result.arg3, "path/to/file".parse::<PathBuf>().unwrap());

    Ok(())
}

#[test]
fn options() -> Result<(), entrance::options::Error> {
    #[derive(Options, Debug, PartialEq)]
    struct Opts {
        verbose: bool,
        version: bool,
        help: bool,
    }

    let args = ["--help", "--verbose", "arg1", "arg2"];
    let mut peekable = args.iter().map(|s| s.to_string()).peekable();
    let opts = Opts::consume(&mut peekable)?;

    assert!(opts.verbose);
    assert!(!opts.version);
    assert!(opts.help);
    assert_eq!(peekable.next(), Some("arg1".to_string()));

    let args = ["--help", "--invalid", "arg1", "arg2"];
    let mut peekable = args.iter().map(|s| s.to_string()).peekable();
    let opts = Opts::consume(&mut peekable);
    assert_eq!(
        opts,
        Err(entrance::options::Error::InvalidLongOption(
            "invalid".to_string()
        ))
    );

    Ok(())
}
