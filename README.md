# ListCrunch

A simple human-readable way to compress redundant sequential data, ported to Rust from [Python](https://github.com/MuckRock/listcrunch).

## Examples

The `listcrunch` crate exposes two functions: `crunch` and `uncrunch`:

```rust
use listcrunch::crunch;

fn main() {
    compressed_string = crunch(vec![["595.0x842.0", "595.0x842.0", "595.0x842.0", "595.0x842.0", "595.0x842.0", "595.0x842.0", "595.0x842.0"]]);
    println!(compressed_string);
    // Returns "595.0x842.0:0-6", meaning 595.0x842.0 appears in indices 0-6 (inclusive),
}
```

```rust
use listcrunch::uncrunch;

fn main() {
    let decompressed = uncrunch("595.0x842.0:0-6").unwrap();
    println!("{:?}", decompressed);
    // Returns ["595.0x842.0", "595.0x842.0", "595.0x842.0", "595.0x842.0", "595.0x842.0", "595.0x842.0", "595.0x842.0"]
}
```

`uncrunch` always returns a vector of string slices (wrapped in a `Result`), which can be further coerced as needed into different types.

[DocumentCloud](https://www.documentcloud.org) uses ListCrunch to represent [page dimensions](https://www.documentcloud.org/help/api/#page-spec). For example, `612.0x792.0:0-447` means 448 pages, all with dimensions of 612 by 792 pixels.
