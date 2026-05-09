# use-luminance

Relative luminance helpers for Rust.

## Example

```rust
use use_luminance::relative_luminance;
use use_rgb::Rgb;

assert_eq!(relative_luminance(Rgb::black()), 0.0);
```
