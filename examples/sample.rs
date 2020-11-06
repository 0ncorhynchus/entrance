use entrance::*;
use std::env;
use std::path::PathBuf;

#[derive(Arguments)]
struct Args {
    #[entrance(description = "Integer argument (i32)")]
    integer: i32,

    #[entrance(description = "Float argument   (f64)")]
    float: f64,

    #[entrance(description = "String argument  (String)")]
    string: String,

    #[entrance(description = "File path list   (Vec<PathBuf>)")]
    #[entrance(variable_argument)]
    files: Vec<PathBuf>,
}

#[derive(Options, PartialEq)]
enum Opts {
    #[entrance(description = "Print the help message")]
    #[entrance(short = 'h')]
    #[entrance(informative(entrance::help))]
    Help,

    #[entrance(description = "Use verbose output")]
    #[entrance(informative(entrance::version))]
    Version,

    #[entrance(description = "Use verbose output")]
    #[entrance(short = 'v')]
    Verbose,
}

type Command = entrance::Command<Opts, Args>;

fn main() {
    let command = Command::new("sample", env!("CARGO_PKG_VERSION"));
    let (opts, args) = command.parse_or_exit(env::args());

    if opts.contains(&Opts::Verbose) {
        println!("--verbose");
    }

    println!("integer: {}", args.integer);
    println!("float:   {}", args.float);
    println!("string:  {}", args.string);
    println!("paths:");
    for path in &args.files {
        println!("    {}", path.display());
    }
}
