pub use entrance_derive::Args;

type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

pub trait Args: Sized {
    fn parse_from<I: Iterator<Item = String>>(args: I) -> Result<Self>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn parse_from() -> Result<()> {
        struct Arguments {
            arg1: String,
            arg2: i32,
            arg3: PathBuf,
        }

        impl Args for Arguments {
            fn parse_from<I: Iterator<Item = String>>(mut args: I) -> Result<Self> {
                args.next();
                Ok(Self {
                    arg1: args.next().unwrap().parse()?,
                    arg2: args.next().unwrap().parse()?,
                    arg3: args.next().unwrap().parse()?,
                })
            }
        }

        let args = ["sample", "arg1", "123", "path/to/file"];
        let result = Arguments::parse_from(args.iter().map(|s| s.to_string()))?;

        assert_eq!(result.arg1, "arg1".to_string());
        assert_eq!(result.arg2, 123);
        assert_eq!(result.arg3, "path/to/file".parse::<PathBuf>().unwrap());

        Ok(())
    }
}
