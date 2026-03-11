[![Crates.io](https://img.shields.io/crates/v/palette-core)](https://crates.io/crates/palette-core)
[![docs.rs](https://img.shields.io/docsrs/palette-core)](https://docs.rs/palette-core)
[![CI](https://github.com/jostled-org/palette-core/actions/workflows/ci.yml/badge.svg)](https://github.com/jostled-org/palette-core/actions/workflows/ci.yml)
[![Downloads](https://img.shields.io/crates/d/palette-core)](https://crates.io/crates/palette-core)
[![deps.rs](https://deps.rs/repo/github/jostled-org/palette-core/status.svg)](https://deps.rs/repo/github/jostled-org/palette-core)
[![License: MIT/Apache-2.0](https://img.shields.io/crates/l/palette-core)](LICENSE-MIT)

Need a color palette? Do you have to hand-roll it?

**Palette Core** is a theme engine that turns TOML palette definitions into CSS, terminal, egui, JSON, and WASM exports.

```rust
use palette_core::preset;

let palette = preset("tokyonight").expect("builtin preset");
let css = palette.to_css();
```

```css
:root {
  --bg: #1A1B26;
  --fg: #C0CAF5;
  --error: #DB4B4B;
  --syn-keyword: #BB9AF7;
  --ed-cursor: #C0CAF5;
  /* ... 98 variables total */
}
```

## Install

```
cargo add palette-core
```

## Usage

### Terminal

```rust
use palette_core::preset;
use palette_core::terminal::to_terminal_theme;

let palette = preset("catppuccin").expect("builtin preset");
let theme = to_terminal_theme(&palette);
```

### Theme variants

Variants inherit from a base and override only what changes.

```toml
[meta]
name = "My Dark Warm"
preset_id = "my_dark_warm"
inherits = "tokyonight"

[base]
background = "#24283b"
```

### Contrast validation

```rust
use palette_core::{preset, ContrastLevel};
use palette_core::contrast::validate_palette;

let palette = preset("tokyonight").expect("builtin preset");
let violations = validate_palette(&palette, ContrastLevel::AaNormal);
```

## Documentation

The [guide](docs/guide.md) covers rendering targets, theme switching, custom presets, color manipulation, platform overrides, and WASM bindings.

31 presets ship built-in — Catppuccin, TokyoNight, Dracula, Nord, Gruvbox, and [more](docs/guide.md#bundled-presets). Golden Hour is an original warm-toned family (light, dusk, twilight) exclusive to palette-core. Optional [feature flags](docs/guide.md#feature-flags) enable `terminal`, `egui`, `snapshot`, `platform`, and `wasm` targets.

## License

Licensed under [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT). Contributions welcome via pull request.
