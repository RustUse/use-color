# use-rgb

RGB color primitives for Rust.

## Example

```rust
use use_rgb::Rgb;

let orange = Rgb::new(255, 69, 0);
assert_eq!(orange.as_tuple(), (255, 69, 0));
```
