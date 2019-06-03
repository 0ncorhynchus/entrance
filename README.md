entrance
========

[![CircleCI](https://circleci.com/gh/0ncorhynchus/entrance.svg?style=svg)](https://circleci.com/gh/0ncorhynchus/entrance)

Type sytem assisted command line argument parser

`entrance` provides type assisted tools for parsing command line arguments

Simple usage
------------

```rust
use entrance::*;
use std::env::args;

#[derive(Options)]
struct Opts {
    #[description = "Print the usage"]
    #[short = 'h']
    help: bool,

    #[description = "Print the version"]
    version: bool,
}

fn main() {
    let command: Command<Opts, ()> = Command::new(env!("CARGO_PKG_NAME"))
        .parse(args())
        .unwrap();

    if command.options().help {
        println!("{}", command.help());
    }

    if command.options().version {
        println!("{} {}", command.name(), env!("CARGO_PKG_VERSION"));
    }
}
```
