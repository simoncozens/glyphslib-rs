# openstep-plist

A Rust parser and serializer for the OpenStep Property List format.

## Overview

OpenStep Property Lists are a text-based data serialization format used by NeXTSTEP, macOS, and various applications including [Glyphs font editor](https://glyphsapp.com/). This crate provides:

- **Parsing**: Convert OpenStep plist text to Rust data structures
- **Serialization**: Convert Rust data structures back to OpenStep plist format
- **Serde integration**: Full support for `serde` serialization/deserialization

## Format

The OpenStep Property List format is similar to JSON but uses a different syntax:

```
{
    name = "Example";
    values = (1, 2, 3);
    nested = {
        key = value;
    };
}
```

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
openstep-plist = "0.1"
```

## Usage

### Parsing

```rust
use openstep_plist::Plist;

let plist_text = r#"{
    name = "Hello";
    count = 42;
}"#;

let plist = Plist::parse(plist_text)?;
```

### With Serde

```rust
use serde::{Deserialize, Serialize};
use openstep_plist;

#[derive(Serialize, Deserialize)]
struct Config {
    name: String,
    count: i32,
}

let plist_text = r#"{ name = "Hello"; count = 42; }"#;
let config: Config = openstep_plist::from_str(plist_text)?;

let serialized = openstep_plist::to_string(&config)?;
```

## Features

- Full support for OpenStep plist data types: dictionaries, arrays, strings, numbers, booleans, and data
- Preserves numeric precision
- Handles quoted and unquoted strings
- Comment support
- Efficient parsing with minimal allocations

## Origins

This crate is inspired by and partially based on the [ascii_plist_derive crate](https://crates.io/crates/ascii_plist_derive) but provides Serde integration and support for serialization as well as deserialization.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
