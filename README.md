# make_u32_from_two_u16

Tiny Rust crate with one job: concatenate two `u16` values into one `u32`.

## What It Does

`concat_u16_to_u32(high, low)` shifts `high` into the upper 16 bits and places `low` in the lower 16 bits.

The function is a `const fn`, so it can also be used in compile-time contexts.

## Example

```rust
use make_u32_from_two_u16::concat_u16_to_u32;

assert_eq!(concat_u16_to_u32(0x1234, 0xABCD), 0x1234ABCD);
```

## Test

```bash
cargo test
```

## Build

```bash
cargo build
```

## Notes

- Library crate only, no CLI.
- Zero dependencies.
- The API is intentionally just one function.
