use entrance::Args;
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
