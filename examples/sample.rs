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

#[derive(VariableArguments)]
struct VarArg {
    #[description = "List of files"]
    files: Box<[PathBuf]>,
}

type Command = entrance::Command<Opts, Args, VarArg>;

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
    println!("files:");
    for file in command.variable_argument().files.iter() {
        println!("    {}", file.display());
    }
}
