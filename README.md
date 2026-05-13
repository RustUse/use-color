# use-color

Composable color primitives for Rust.

`use-color` provides small utilities for RGB values, hex colors, HSL conversion, relative luminance, and contrast checks.

## RustUse relationship

- `use-color` is a sibling set to `use-math`, `use-text`, and `use-wave`.
- Crates stay one layer deep.
- Each crate should be independently useful.

## Workspace crates

- [`use-rgb`](./crates/use-rgb): RGB color primitives.
- [`use-hex-color`](./crates/use-hex-color): Hex color parsing and formatting.
- [`use-hsl`](./crates/use-hsl): HSL color primitives and RGB conversion.
- [`use-luminance`](./crates/use-luminance): Relative luminance helpers.
- [`use-contrast`](./crates/use-contrast): WCAG-style contrast ratio helpers.

## Examples

```rust
use use_contrast::{contrast_ratio, passes_aa};
use use_hex_color::HexColor;
use use_hsl::Hsl;
use use_luminance::relative_luminance;
use use_rgb::Rgb;

let orange = Rgb::new(255, 69, 0);
let white = Rgb::white();

let hex = HexColor::from_rgb(orange);
let parsed = HexColor::parse("#FF4500").unwrap();

let hsl = Hsl::from_rgb(orange);
let luminance = relative_luminance(orange);

let ratio = contrast_ratio(orange, white);
let accessible = passes_aa(orange, white);

assert_eq!(hex.as_str(), "#FF4500");
assert_eq!(parsed.to_rgb(), orange);
assert!(hsl.h() >= 0.0);
assert!(luminance >= 0.0);
assert!(ratio >= 1.0);
assert_eq!(accessible, passes_aa(orange, white));
```
