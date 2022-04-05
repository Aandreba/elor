# Either Left Or Right (elor)
[![Crate](https://img.shields.io/crates/v/elor.svg)](https://crates.io/crates/elor)
[![API](https://docs.rs/elor/badge.svg)](https://docs.rs/elor)

elor is a ```no_std``` Rust crate with only one simple type; **```Either```**.
It represents a value that's of one type or another, and implements various functionalities depending on
the attributes of those types

## Features
| Name            | Description                                   | Dependencies                                |
| --------------- | --------------------------------------------- | ------------------------------------------- |
| ```serialize``` | Allow for serialization and deserialization   | [serde](https://github.com/serde-rs/serde)  |
| ```random```    | Allows generating random ```Either```'s       | [rand](https://github.com/rust-random/rand) |
| ```async```     | Allows async polling of async ```Either```'s  | None                                        |