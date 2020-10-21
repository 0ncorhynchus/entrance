use entrance::*;
use std::env;
use std::path::PathBuf;

#[derive(Arguments)]
struct Args {
    #[entrance(description = "Integer argument (i32)")]
    integer: i32,

    #[entrance(description = "Float argument   (f64)")]
    float: f64,

    #[entrance(description = "String argument  (String)")]
    string: String,

    #[entrance(description = "File path list   (Vec<PathBuf>)")]
    #[entrance(variable_argument)]
    files: Vec<PathBuf>,
}

#[derive(Options, PartialEq)]
enum Opts {
    #[entrance(description = "Print the help message")]
    #[entrance(short = 'h')]
    #[entrance(informative(entrance::Command::help))]
    Help,

    #[entrance(description = "Use verbose output")]
    #[entrance(short = 'v')]
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
            command.help();
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
