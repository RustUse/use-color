# use-hex-color

Hex color parsing and formatting for Rust.

## Example

```rust
use use_hex_color::HexColor;
use use_rgb::Rgb;

let parsed = HexColor::parse("#fff").unwrap();
assert_eq!(parsed.as_str(), "#FFFFFF");
assert_eq!(parsed.to_rgb(), Rgb::white());
```
