use entrance::Args;
use std::env;

#[derive(Args)]
struct Arguments {
    integer: i32,
    float: f64,
    string: String,
}

fn main() {
    let args = match Arguments::parse_from(env::args()) {
        Ok(args) => args,
        Err(err) => {
            eprintln!("\x1b[31merror:\x1b[m {}", err);
            std::process::exit(1);
        }
    };
    println!("integer: {}", args.integer);
    println!("float:   {}", args.float);
    println!("string:  {}", args.string);
}
