# Either Left Or Right (elor)
[![Crate](https://img.shields.io/crates/v/elor.svg)](https://crates.io/crates/elor)
[![API](https://docs.rs/elor/badge.svg)](https://docs.rs/elor)

elor is a ```no_std``` Rust crate with one simple type; **```Either```**.
It represents a value that's of one type or another, and implements various functionalities depending on
the attributes of those types

## Features
| Name            | Description                                                                                                    | Dependencies                                                                  |
| --------------- | -------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------- |
| ```std```       | Allows the usage of standard library functionality. Enabled by default and mutually exclusive with ```alloc``` | [Standard library](https://github.com/rust-lang/rust/tree/master/library/std) |
| ```alloc```     | Allows for heap allocation in the absence of the standard library. Mutually exclusive with ```std```           | [Alloc library](https://github.com/rust-lang/rust/tree/master/library/alloc)  |
| ```serialize``` | Allow for serialization and deserialization                                                                    | [serde](https://github.com/serde-rs/serde)                                    |
| ```random```    | Allows generating random ```Either```'s                                                                        | [rand](https://github.com/rust-random/rand)                                   |
| ```async```     | Allows async polling of async ```Either```'s                                                                   | [futures](https://github.com/rust-lang/futures-rs)                            |
| ```macro```     | Allows for the implementation of functionality geared towards the development of procedural macros             | [quote](https://github.com/dtolnay/quote), [syn](https://github.com/dtolnay/syn) and [proc_macro2](https://github.com/dtolnay/proc-macro2) |