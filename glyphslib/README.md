# glyphslib

A Rust library for reading, writing, and manipulating Glyphs font source files (`.glyphs` and `.glyphspackage`).

## Features

- **Full format support**: Read and write both Glyphs 2 and Glyphs 3 file formats
- **Type-safe**: Strongly typed structures for all font data
- **Unified interface**: Work with either format through the `GlyphsFile` trait
- **Format conversion**: Convert between Glyphs 2 and Glyphs 3 formats
- **Comprehensive**: Access and modify all font data including masters, instances, glyphs, layers, paths, components, anchors, and more

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
glyphslib = "0.1"
```

## Quick Start

```rust
use glyphslib::{Font, GlyphsFile};
use std::path::Path;

// Load a Glyphs file
let font = Font::load(Path::new("MyFont.glyphs"))?;

// Access font properties
let font_ref = font.as_ref();
println!("Family: {}", font_ref.family_name());
println!("Masters: {}", font_ref.masters().len());
println!("Glyphs: {}", font_ref.glyphs().len());

// Iterate through glyphs
for glyph in font_ref.glyphs() {
    println!("  {}: {} layers", glyph.name(), glyph.layers().len());
}
```

## See also

[`glyphs-reader`](https://crates.io/crates/glyphs-reader) is a similar crate for reading Glyphs files, but does not preserve the difference between Glyphs 2 and Glyphs 3 formats, nor does it support writing.

## Documentation

For detailed documentation and examples, see the [API documentation](https://docs.rs/glyphslib).

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
