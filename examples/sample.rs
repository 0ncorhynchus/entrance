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
}

type Command = entrance::Command<DefaultInformativeOption, Opts, Args>;

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
        CallType::Informative(info_opt) => {
            match info_opt {
                DefaultInformativeOption::Help => {
                    println!("{}", command.help());
                }
                DefaultInformativeOption::Version => {
                    println!("sample 0.1.0");
                }
            };
            return;
        }
        CallType::Normal(opts, args) => {
            println!("--verbose: {}", opts.verbose);
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
