# make_u32_from_two_u16

Tiny Rust crate with one job: concatenate two `u16` values into one `u32`.

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
