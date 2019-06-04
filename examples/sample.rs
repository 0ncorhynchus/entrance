use entrance::{Arguments, Command, Options};
use std::env;

#[derive(Arguments)]
struct Args {
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
    let command: Command<Opts, Args, ()> = match Command::new("sample").parse(env::args()) {
        Ok(args) => args,
        Err(err) => {
            eprintln!("\x1b[31merror:\x1b[m {}", err);
            std::process::exit(1);
        }
    };
    println!("--verbose: {}", command.options().verbose);
    println!("--version: {}", command.options().version);
    println!("--help: {}", command.options().help);
    println!("integer: {}", command.arguments().integer);
    println!("float:   {}", command.arguments().float);
    println!("string:  {}", command.arguments().string);
}
