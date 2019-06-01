use entrance::{Args, Command, Options};
use std::env;

#[derive(Args)]
struct Arguments {
    integer: i32,
    float: f64,
    string: String,
}

#[derive(Options)]
struct Opts {
    verbose: bool,
    version: bool,
    help: bool,
}

fn main() {
    let command: Command<Opts, Arguments> = match Command::new("sample").parse_args(env::args()) {
        Ok(args) => args,
        Err(err) => {
            eprintln!("\x1b[31merror:\x1b[m {}", err);
            std::process::exit(1);
        }
    };
    println!("--verbose: {}", command.options().verbose);
    println!("--version: {}", command.options().version);
    println!("--help: {}", command.options().help);
    println!("integer: {}", command.args().integer);
    println!("float:   {}", command.args().float);
    println!("string:  {}", command.args().string);
}
