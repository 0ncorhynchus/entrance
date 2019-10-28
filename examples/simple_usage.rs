use entrance::*;
use std::env::args;

type Command = entrance::Command<DefaultInformativeOption, (), ()>;

fn main() {
    let command = Command::new(env!("CARGO_PKG_NAME"));

    match command.parse(args()).unwrap() {
        CallType::Informative(info_opt) => match info_opt {
            DefaultInformativeOption::Help => {
                println!("{}", command.help());
            }
            DefaultInformativeOption::Version => {
                println!("{} {}", command.name(), env!("CARGO_PKG_VERSION"));
            }
        },
        _ => {}
    }
}
