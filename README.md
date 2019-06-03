entrance
========

[![CircleCI](https://circleci.com/gh/0ncorhynchus/entrance.svg?style=svg)](https://circleci.com/gh/0ncorhynchus/entrance)

Simple usage
============

```rust
use entrance::*;
use std::env::args;

#[derive(Options)]
struct Opts {
    #[description = "Print the usage"]
    #[short = 'h']
    help: bool,
    #[description = "Print the version"]
    #[short = 'v']
    version: bool,
}

fn main() {
    let command: Command<Opts, ()> = Command::new(env!("CARGO_PKG_NAME"))
        .parse_args(args())
        .unwrap();

    if command.options().help {
        println!("{}", command.help());
    }

    if command.options().version {
        println!("{} {}", command.name(), env!("CARGO_PKG_VERSION"));
    }
}
```
