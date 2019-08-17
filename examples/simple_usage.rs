use entrance::*;
use std::env::args;

fn main() {
    let command: Command<DefaultInformativeOption, (), ()> =
        Command::new(env!("CARGO_PKG_NAME")).parse(args()).unwrap();

    match command.call_type() {
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
