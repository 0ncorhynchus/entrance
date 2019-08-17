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
use std::env::args;

fn main() {
    let command: Command<DefaultInformativeOption, (), ()> =
        Command::new(env!("CARGO_PKG_NAME")).parse(args()).unwrap();

    match command.call_type() {
        CallType::Informative(info_opt) => match info_opt {
            DefaultInformativeOption::Help => {
                println!("{}", command.help());
            }
            DefaultInformativeOption::Version => {
                println!("{} {}", command.name(), env!("CARGO_PKG_VERSION"));
            }
        },
        _ => {}
    }
}
```

Structs and traits
------------------

### Command

This struct provides tools for parsing command line arguments.

Before parsing command line arguments, it is necessary to create the instance
with the associated function `new`.
Then, use `parse` or `parse_options` and `parse_arguments`.

If arguments should be parsed after checking options, use `parse_options` and `parse_arguments`.
Otherwise, use `parse` simply.

### Options

A derive macro is available for this.

Limitation: the macro supports only the struct with `bool` members.

### Arguments

A derive macro is available for this.

Limitation: the macro supports only the struct with members implementing `FromStr`.
