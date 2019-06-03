use entrance::{Arguments, Options, OptionItem};
use std::path::PathBuf;

#[test]
fn struct_with_named_fields() -> entrance::Result<()> {
    #[derive(Arguments)]
    struct Args {
        arg1: String,
        arg2: i32,
        arg3: PathBuf,
    }

    let args = ["arg1", "123", "path/to/file"];
    let result = Args::parse(args.iter().map(|s| s.to_string()))?;

    assert_eq!(result.arg1, "arg1".to_string());
    assert_eq!(result.arg2, 123);
    assert_eq!(result.arg3, "path/to/file".parse::<PathBuf>().unwrap());

    Ok(())
}

#[test]
fn options() -> Result<(), entrance::OptionError> {
    #[derive(Options, Debug, PartialEq)]
    struct Opts {
        verbose: bool,
        version: bool,
        help: bool,
    }

    let options = vec![
        OptionItem::Long("help".to_string()),
        OptionItem::Long("verbose".to_string()),
    ];
    let opts = Opts::parse(options.into_iter())?;

    assert!(opts.verbose);
    assert!(!opts.version);
    assert!(opts.help);

    let options = vec![
        OptionItem::Long("help".to_string()),
        OptionItem::Long("invalid".to_string()),
    ];
    let opts = Opts::parse(options.into_iter());
    assert_eq!(
        opts,
        Err(entrance::OptionError::InvalidLongOption(
            "invalid".to_string()
        ))
    );

    Ok(())
}
