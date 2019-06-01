use entrance::{Args, Command};
use std::env;

#[derive(Args)]
struct Arguments {
    integer: i32,
    float: f64,
    string: String,
}

fn main() {
    let command: Command<(), Arguments> = match Command::new("sample").parse_args(env::args()) {
        Ok(args) => args,
        Err(err) => {
            eprintln!("\x1b[31merror:\x1b[m {}", err);
            std::process::exit(1);
        }
    };
    println!("integer: {}", command.args().integer);
    println!("float:   {}", command.args().float);
    println!("string:  {}", command.args().string);
}
