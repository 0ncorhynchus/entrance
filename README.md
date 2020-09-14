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

type Command = entrance::Command<DefaultInformativeOption, (), ()>;

fn main() {
    let command = Command::new(env!("CARGO_PKG_NAME"));

    match command.parse(args()).unwrap() {
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
with the associated function `new` then, call `parse` of the instance.

### InformativeOption

A derive macro is available for this.

Limitation: the macro supports only the enum with unit variants.

### Options

A derive macro is available for this.

Limitation: the derive macro supports only an Enum whose variants don't have any field.

### Arguments

A derive macro is available for this.

Limitation: the macro supports only the struct with members implementing `FromStr`.
