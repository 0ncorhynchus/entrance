use entrance::*;
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

struct MyCommand;

impl Command for MyCommand {
    type Opts = Opts;
    type Args = Args;

    fn exec(_info: &CommandInfo<Self>, opts: Vec<Self::Opts>, args: Self::Args) {
        if opts.contains(&Opts::Verbose) {
            println!("enabled the verbose output");
        }

        println!("1st argument: \"{}\"", args.num);
        println!("2nd argument: \"{}\"", args.file.display());
    }
}

fn main() {
    MyCommand::build("sample", env!("CARGO_PKG_VERSION"))
        .parse(env::args())
        .expect("Failed to parse command line arguments")
        .exec();
}
