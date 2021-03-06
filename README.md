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
use entrance::{Arguments, Options};
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

type Command = entrance::Command<Opts, Args>;

fn main() {
    let command = Command::new(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    let (opts, args) = command.parse_or_exit(env::args());

    if opts.contains(&Opts::Verbose) {
        println!("enabled the verbose output");
    }

    println!("1st argument: \"{}\"", args.num);
    println!("2nd argument: \"{}\"", args.file.display());
}
```

Structs and traits
------------------

### Command

This struct provides tools for parsing command line arguments.

Before parsing command line arguments, it is necessary to create the instance
with the associated function `new` then, call `parse` of the instance.

### Options

A derive macro is available for this.

Limitation: the derive macro supports only an Enum whose variants don't have any field.

### Arguments

A derive macro is available for this.

Limitation: the macro supports only the struct with members implementing `FromStr`.
