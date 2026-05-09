# use-hsl

HSL color primitives and RGB conversion for Rust.

## Example

```rust
use use_hsl::Hsl;
use use_rgb::Rgb;

let hsl = Hsl::from_rgb(Rgb::red());
assert_eq!(hsl.to_rgb(), Rgb::red());
```
