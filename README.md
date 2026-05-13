# use-color

Composable primitive color utilities for Rust.

`use-color` is the color-specific RustUse set for small, composable helpers
around RGB values and hex colors. It focuses on lightweight representation and
conversion without introducing full palette management or rendering engines.

## Scope

- RGB color values
- hex color parsing and formatting
- grayscale checks
- relative luminance for sRGB colors
- named color constants for common base colors

## Workspace crates

| Crate       | Purpose                                        |
| ----------- | ---------------------------------------------- |
| `use-color` | Composable primitive color utilities for Rust. |

## Installation

Crates.io:

```toml
[dependencies]
use-color = "0.1"
```

Local workspace or path dependency:

```toml
[dependencies]
use-color = { path = "crates/use-color" }
```

## Examples

Parse and format a hex color:

```rust
use use_color::prelude::*;

let color = parse_hex_rgb("#3366CC").unwrap();

assert_eq!(color.to_hex_rgb(), "#3366CC");
```

Check whether a color is grayscale:

```rust
use use_color::prelude::*;

assert!(Rgb::new(80, 80, 80).is_grayscale());
assert!(!Rgb::new(80, 81, 80).is_grayscale());
```

Measure luminance:

```rust
use use_color::prelude::*;

assert!(WHITE.relative_luminance() > BLACK.relative_luminance());
```

## Status

Early v0.1 API.
