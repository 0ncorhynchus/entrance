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

    let options = vec![
        OptionItem::Long("help".to_string()),
        OptionItem::Long("verbose".to_string()),
    ];
    let opts: entrance::Result<Vec<_>> = Opts::parse(options.into_iter()).into_iter().collect();
    let opts = opts?;

    assert!(opts.contains(&Opts::Verbose));
    assert!(!opts.contains(&Opts::Version));
    assert!(opts.contains(&Opts::Help));

    let options = vec![
        OptionItem::Long("help".to_string()),
        OptionItem::Long("invalid".to_string()),
    ];
    let opts = Opts::parse(options.into_iter());
    assert_eq!(opts.len(), 2);

    assert!(opts[0].is_ok());
    assert_eq!(opts[0].as_ref().unwrap(), &Opts::Help);

    assert!(opts[1].is_err());
    assert_eq!(
        opts[1].as_ref().unwrap_err().kind(),
        entrance::ErrorKind::InvalidOption
    );

    Ok(())
}
