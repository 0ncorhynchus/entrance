entrance
========

[![Crates.io](https://img.shields.io/crates/v/entrance.svg)](https://crates.io/crates/entrance)
[![Document](https://docs.rs/entrance/badge.svg)](https://docs.rs/entrance)
[![CircleCI](https://circleci.com/gh/0ncorhynchus/entrance.svg?style=shield)](https://circleci.com/gh/0ncorhynchus/entrance)

Type sytem assisted command line argument parser

`entrance` provides type assisted tools for parsing command line arguments.

Simple usage
------------

```rust
use entrance::*;
use std::env;
use std::path::PathBuf;

#[derive(Options, PartialEq)]
enum Opts {
    #[entrance(description = "Print help message")]
    #[entrance(short = 'h')]
    #[entrance(informative(entrance::help))]
    Help,

    #[entrance(description = "Print version infomation")]
    #[entrance(informative(entrance::version))]
    Version,

    #[entrance(description = "Use verbose output")]
    #[entrance(short = 'v')]
    Verbose,
}

#[derive(Arguments)]
struct Args {
    #[entrance(description = "The number of lines")]
    num: f64,

    #[entrance(description = "Path to a file")]
    file: PathBuf,
}

struct MyCommand;

impl Command for MyCommand {
    type Opts = Opts;
    type Args = Args;

    fn exec(_info: &CommandInfo<Self>, opts: Vec<Opts>, args: Args) {
        if opts.contains(&Opts::Verbose) {
            println!("enabled the verbose output");
        }

        println!("1st argument: \"{}\"", args.num);
        println!("2nd argument: \"{}\"", args.file.display());
    }
}

fn main() {
    MyCommand::build("sample", env!("CARGO_PKG_VERSION"))
        .parse(env::args())
        .expect("Failed to parse command line arguments")
        .exec();
}
```

When you compile this code and execute it with a help option '--help', you will obtain the below output:

```console
$ cargo run -- --help
USAGE:
    sample [OPTIONS] <num> <file>

OPTIONS:
    -h, --help       Print help message
        --version    Print version infomation
    -v, --verbose    Use verbose output

ARGS:
    num     The number of lines
    file    Path to a file
```
