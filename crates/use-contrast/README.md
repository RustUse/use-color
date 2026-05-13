# use-contrast

WCAG-style contrast ratio helpers for Rust.

## Example

```rust
use use_contrast::{contrast_ratio, passes_aa};
use use_rgb::Rgb;

let ratio = contrast_ratio(Rgb::black(), Rgb::white());
assert!(ratio > 20.0);
assert!(passes_aa(Rgb::black(), Rgb::white()));
```
