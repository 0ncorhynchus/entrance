use entrance::*;
use std::env::args;

#[derive(Options)]
struct Opts {
    help: bool,
    version: bool,
}

fn main() {
    let command: Command<Opts, ()> = Command::new(env!("CARGO_PKG_NAME"))
        .parse_args(args())
        .unwrap();

    if command.options().help {
        println!("{}", command.help());
    }

    if command.options().version {
        println!("{} {}", command.name(), env!("CARGO_PKG_VERSION"));
    }
}
