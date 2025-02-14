# ListCrunch

A simple human-readable way to compress redundant sequential data, ported to Rust from [Python](https://github.com/MuckRock/listcrunch).

## Examples

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
    let decompressed = uncrunch("595.0x842.0:0-6")
    println!("{:?}", decompressed);
    // Returns ["595.0x842.0", "595.0x842.0", "595.0x842.0", "595.0x842.0", "595.0x842.0", "595.0x842.0", "595.0x842.0"]
}
```

`uncrunch` always returns strings, which can be further coerced, as needed, into different types.
