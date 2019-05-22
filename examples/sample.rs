use entrance::Args;
use std::env;

#[derive(Args)]
struct Arguments {
    integer: i32,
    float: f64,
    string: String,
}

fn main() {
    let args = Arguments::parse_from(env::args()).unwrap();
    println!("integer: {}", args.integer);
    println!("float:   {}", args.float);
    println!("string:  {}", args.string);
}
