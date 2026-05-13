# use-color

Composable primitive color utilities for Rust.

`use-color` provides small helpers for RGB values, hex parsing and formatting,
and relative luminance.

## Examples

```rust
use use_color::prelude::*;

let color = parse_hex_rgb("#3366CC").unwrap();

assert_eq!(color.to_hex_rgb(), "#3366CC");
assert!(!color.is_grayscale());
```
