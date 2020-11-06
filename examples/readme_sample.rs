use entrance::{Arguments, Options};
use std::env;
use std::path::PathBuf;

#[derive(Options, PartialEq)]
enum Opts {
    #[entrance(description = "Print help message")]
    #[entrance(short = 'h')]
    #[entrance(informative(entrance::help))]
    Help,

    #[entrance(description = "Print version infomation")]
    #[entrance(informative(entrance::version))]
    Version,

    #[entrance(description = "Use verbose output")]
    #[entrance(short = 'v')]
    Verbose,
}

#[derive(Arguments)]
struct Args {
    #[entrance(description = "The number of lines")]
    num: f64,

    #[entrance(description = "Path to a file")]
    file: PathBuf,
}

type Command = entrance::Command<Opts, Args>;

fn main() {
    let command = Command::new(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    let (opts, args) = command.parse_or_exit(env::args());

    if opts.contains(&Opts::Verbose) {
        println!("enabled the verbose output");
    }

    println!("1st argument: \"{}\"", args.num);
    println!("2nd argument: \"{}\"", args.file.display());
}
