use crate::Result;

#[derive(Debug, Clone, Copy)]
pub struct Arg {
    pub name: &'static str,
    pub description: &'static str,
}

/// A trait for parsing and containing arguments.
///
/// # Example
/// ```
/// use entrance::Arguments;
/// use std::path::PathBuf;
///
/// #[derive(Arguments)]
/// struct Args {
///     #[description = "The number of lines"]
///     num: f64,
///
///     #[description = "Path to a file"]
///     file: PathBuf,
/// }
/// ```
///
/// # Limitation
/// The derive macro for `Arguments` supports only a struct with named fields.
/// Additionally, these fields should implement `FromStr`.
pub trait Arguments: Sized {
    fn parse<I: Iterator<Item = String>>(args: &mut I) -> Result<Self>;

    /// This associated function is for `HelpDisplay`.
    fn spec() -> &'static [Arg];
}

impl Arguments for () {
    fn parse<I: Iterator<Item = String>>(_args: &mut I) -> Result<Self> {
        Ok(())
    }

    fn spec() -> &'static [Arg] {
        &[]
    }
}

/// A trait for parsing and containing variable arguments.
///
/// # Example
/// ```
/// use entrance::VariableArguments;
/// use std::path::PathBuf;
///
/// #[derive(VariableArguments)]
/// struct VarArgs {
///     #[description = "List of files"]
///     files: Vec<PathBuf>,
/// }
/// ```
///
/// # Limitation
/// The derive macro for `VariableArguments` supports only a struct with a single named field.
/// Additionally, this field should implement `From<Vec<T>>` where `T` implements `FromStr`.
pub trait VariableArguments: Sized {
    fn parse<I: Iterator<Item = String>>(args: &mut I) -> Result<Self>;

    /// This associated function is for `HelpDisplay`.
    fn spec() -> Option<Arg>;
}

impl VariableArguments for () {
    fn parse<I: Iterator<Item = String>>(_args: &mut I) -> Result<Self> {
        Ok(())
    }

    fn spec() -> Option<Arg> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arguments_parse() -> Result<()> {
        let args = ["arg1", "123", "path/to/file"];
        let mut args = args.iter().map(|s| s.to_string());
        let _ = <() as Arguments>::parse(&mut args)?;

        assert_eq!(args.next(), Some("arg1".to_string()));

        Ok(())
    }

    #[test]
    fn arguments_spec() {
        assert_eq!(<() as Arguments>::spec().len(), 0);
    }

    #[test]
    fn variable_argument_parse() -> Result<()> {
        let args = ["arg1", "123", "path/to/file"];
        let mut args = args.iter().map(|s| s.to_string());
        let _ = <() as VariableArguments>::parse(&mut args)?;

        assert_eq!(args.next(), Some("arg1".to_string()));

        Ok(())
    }

    #[test]
    fn variable_argument_spec() {
        assert!(<() as VariableArguments>::spec().is_none());
    }
}
