use entrance::{Arguments, OptionItem, Options, VariableArguments};
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
    let result = Args::parse(&mut args.iter().map(|s| s.to_string()))?;

    assert_eq!(result.arg1, "arg1".to_string());
    assert_eq!(result.arg2, 123);
    assert_eq!(result.arg3, PathBuf::from("path/to/file"));

    Ok(())
}

#[test]
fn options() -> Result<(), entrance::Error> {
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
    match opts {
        Ok(_) => {
            panic!("Err(InvalidOption) is expected.");
        }
        Err(error) => match error {
            entrance::Error::InvalidOption => {}
            _ => {
                panic!("Err(InvalidOption) is expected.");
            }
        },
    }

    Ok(())
}

#[test]
fn variable_argument() -> entrance::Result<()> {
    #[derive(VariableArguments)]
    struct VarArg {
        files: Vec<PathBuf>,
    }

    let args = ["path/to/file", "./test.txt"];
    let result = VarArg::parse(&mut args.iter().map(|s| s.to_string()))?;

    assert_eq!(
        result.files,
        [PathBuf::from("path/to/file"), PathBuf::from("./test.txt")]
    );

    Ok(())
}
