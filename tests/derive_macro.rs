use entrance::{Arguments, OptionItem, Options};
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
    enum Opts {
        Verbose,
        Version,
        Help,
    }

    let option = Opts::parse(OptionItem::Long("verbose".to_string()))?;
    assert_eq!(option, Opts::Verbose);

    let option = Opts::parse(OptionItem::Long("version".to_string()))?;
    assert_eq!(option, Opts::Version);

    let option = Opts::parse(OptionItem::Long("help".to_string()))?;
    assert_eq!(option, Opts::Help);

    let option = Opts::parse(OptionItem::Long("invalid".to_string()));
    assert!(option.is_err());
    assert_eq!(
        option.unwrap_err().kind(),
        entrance::ErrorKind::InvalidOption
    );

    Ok(())
}
