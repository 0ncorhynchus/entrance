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

#[derive(Options)]
struct Opts {
    #[description = "Use verbose output"]
    #[short = 'v']
    verbose: bool,

    #[description = "Print version information"]
    version: bool,

    #[description = "Print help message"]
    #[short = 'h']
    help: bool,
}

type Command = entrance::Command<Opts, Args>;

fn main() {
    let command = match Command::new("sample").parse_options(env::args()) {
        Ok(command) => command,
        Err(err) => {
            eprintln!("\x1b[31merror:\x1b[m {}", err);
            std::process::exit(1);
        }
    };

    if command.options().help {
        println!("{}", command.help());
        return;
    }

    if command.options().version {
        println!("sample 0.1.0");
        return;
    }

    let command = match command.parse_arguments() {
        Ok(command) => command,
        Err(err) => {
            eprintln!("\x1b[31merror:\x1b[m {}", err);
            std::process::exit(2);
        }
    };

    println!("--verbose: {}", command.options().verbose);
    println!("--version: {}", command.options().version);
    println!("--help:    {}", command.options().help);
    println!("integer: {}", command.arguments().integer);
    println!("float:   {}", command.arguments().float);
    println!("string:  {}", command.arguments().string);
    println!("paths:");
    for path in &command.arguments().files {
        println!("    {}", path.display());
    }
}
