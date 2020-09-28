use entrance::*;
use std::env;
use std::path::PathBuf;

#[derive(Arguments)]
struct Args {
    #[description = "Integer argument (i32)"]
    integer: i32,

    #[description = "Float argument   (f64)"]
    float: f64,

    #[description = "String argument  (String)"]
    string: String,

    #[description = "File path list   (Vec<PathBuf>)"]
    #[variable_argument]
    files: Vec<PathBuf>,
}

#[derive(Options, PartialEq)]
enum Opts {
    #[description = "Print the help message"]
    #[short = 'h']
    #[informative]
    Help,

    #[description = "Use verbose output"]
    #[short = 'v']
    Verbose,
}

type Command = entrance::Command<Opts, Args>;

fn main() {
    let command = Command::new("sample");
    let call_type = match command.parse(env::args()) {
        Ok(call_type) => call_type,
        Err(err) => {
            eprintln!("\x1b[31merror:\x1b[m {}", err);
            std::process::exit(1);
        }
    };

    match call_type {
        CallType::Informative(_) => {
            println!("{}", command.help());
            return;
        }
        CallType::Normal(opts, args) => {
            if opts.contains(&Opts::Verbose) {
                println!("--verbose");
            }
            println!("integer: {}", args.integer);
            println!("float:   {}", args.float);
            println!("string:  {}", args.string);
            println!("paths:");
            for path in &args.files {
                println!("    {}", path.display());
            }
        }
    }
}
