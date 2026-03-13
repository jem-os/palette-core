# Guide

Presets, rendering targets, theme switching, custom themes, and utilities. For a quick start, see the [README](../README.md). For CSS variable names, see the [CSS variables reference](css-variables.md).

## Loading presets

Built-in presets are compiled into the binary. `preset()` returns `Option<Palette>` — `None` only if the ID doesn't match a builtin.

```rust
use palette_core::preset;

let palette = preset("tokyonight").expect("builtin preset");
```

### `preset()` vs `load_preset()`

| | `preset(id)` | `load_preset(id)` |
|---|---|---|
| Returns | `Option<Palette>` | `Result<Palette, PaletteError>` |
| Unknown ID | `None` | `Err(UnknownPreset)` |
| Use when | Loading a known builtin by name | You need the error type (e.g. to propagate with `?`) |

Both resolve inheritance for variant presets (e.g. `tokyonight_storm` inherits from `tokyonight`).

For user-provided TOML files, use `load_preset_file()` or a `Registry` — those paths can genuinely fail (missing file, bad TOML, broken inheritance chain).

### Fallback when loading fails

`Palette` implements `Default` — a neutral dark palette with base, semantic, and surface colors. Use it as a safe fallback:

```rust
use palette_core::Palette;

let palette = reg.load(&user_choice).unwrap_or_default();
```

Or combine with `preset()` for a two-tier fallback:

```rust
use palette_core::preset;
use palette_core::Palette;

let palette = preset(&user_choice)
    .or_else(|| preset("tokyonight"))
    .unwrap_or_default();
```

The default palette covers `base`, `semantic`, and `surface` slots. Syntax, editor, terminal, and diff slots are `None` — downstream renderers should apply their own defaults for those.

## Resolved palettes

`Palette` fields are `Option<Color>` — absent slots mean the theme defers to the renderer. Call `resolve()` to fill all gaps from `Palette::default()`, producing a `ResolvedPalette` where every slot is a concrete `Color`.

```rust
use palette_core::preset;

let palette = preset("tokyonight").expect("builtin preset");
let resolved = palette.resolve();
// resolved.base.background is Color, not Option<Color>
```

Use `resolve_with()` to fill gaps from a custom fallback instead of the built-in default.

```rust
let custom_fallback = palette_core::preset("nord").expect("builtin preset");
let resolved = palette.resolve_with(&custom_fallback);
```

## Style modifiers

Syntax tokens can carry bold, italic, and underline modifiers alongside their colors. These live in the `[syntax_style]` TOML section and the `Palette.syntax_style` field.

```toml
[syntax_style]
keywords = "italic"
comments = "italic"
comments_doc = "bold,italic"
```

```rust
use palette_core::preset;

let palette = preset("tokyonight").expect("builtin preset");
for (name, style) in palette.syntax_style.populated_slots() {
    println!("{name}: {style}");
}
```

Style modifiers follow the same parent→child fallback chain as syntax colors. Setting `keywords = "italic"` applies italic to `keywords_control`, `keywords_import`, and `keywords_operator` unless they have their own explicit style.

Resolved styles (`ResolvedSyntaxStyles`) fill absent slots with `StyleModifiers::default()` (no modifiers). CSS export appends `--syn-*-style` variables for non-empty modifiers.

## Rendering targets

### CSS

```rust
use palette_core::preset;

let palette = preset("tokyonight").expect("builtin preset");
let css = palette.to_css();
```

For custom selectors or prefixed variables:

```rust
let scoped = palette.to_css_scoped("[data-theme=\"tokyonight\"]", None);
let prefixed = palette.to_css_scoped(":root", Some("app")); // --app-bg, --app-fg, ...
```

See the [CSS variables reference](css-variables.md) for the full variable list.

### Terminal (ratatui)

Requires the `terminal` feature.

```rust
use palette_core::preset;
use palette_core::terminal::to_terminal_theme;

let palette = preset("catppuccin").expect("builtin preset");
let theme = to_terminal_theme(&palette);
// theme.base.background, theme.syntax.keywords, etc.
```

For a resolved theme where every slot is a concrete `RatatuiColor`:

```rust
use palette_core::terminal::to_resolved_terminal_theme;

let resolved = palette.resolve();
let theme = to_resolved_terminal_theme(&resolved);
// theme.base.background is RatatuiColor, not Option<RatatuiColor>
```

#### Accent color cycling

`chromatic()` returns the 12 non-grayscale ANSI colors for cycling across panels or chart series:

```rust
let colors = theme.terminal_ansi.chromatic();
for (i, panel) in panels.iter().enumerate() {
    let accent = colors[i % colors.len()];
    // ...
}
```

#### Style builder

`style(fg, bg)` is shorthand for `Style::default().fg(fg).bg(bg)`:

```rust
use palette_core::terminal::style;

let base = style(theme.base.foreground, theme.base.background);
```

### egui

Requires the `egui` feature.

```rust
use palette_core::preset;
use palette_core::egui::to_egui_visuals;

let palette = preset("github_dark").expect("builtin preset");
ctx.set_visuals(to_egui_visuals(&palette));
```

### JSON

Requires the `snapshot` feature.

```rust
use palette_core::preset;

let palette = preset("nord").expect("builtin preset");
let json = palette.to_json()?;
```

### WASM

Requires the `wasm` feature.

```js
import { preset, loadPresetCss } from "palette-core";

const palette = preset("tokyonight");   // returns palette or undefined
console.log(palette.name());            // "TokyoNight (Night)"
console.log(palette.toCss());           // :root { --bg: ...; --fg: ...; }

const css = loadPresetCss("dracula");   // :root { ... }
```

## Theme switching with Registry

`Registry` holds all loaded presets in one namespace. Load a default at startup. Let users pick from the list.

```rust
use palette_core::Registry;

let reg = Registry::new();

// Populate a settings menu
for info in reg.list() {
    println!("{} ({})", info.name, info.style);
}

// Load the user's choice
let palette = reg.load("catppuccin")?;
```

### CSS — generate all themes for live switching

```rust
use palette_core::Registry;

let reg = Registry::new();
let mut css = String::new();
for info in reg.list() {
    let palette = reg.load(&info.id)?;
    let selector = format!("[data-theme=\"{}\"]", info.id);
    css.push_str(&palette.to_css_scoped(&selector, None));
}
```

Switch themes in the browser by setting a data attribute:

```js
document.documentElement.dataset.theme = "catppuccin";
```

Reference the variables in CSS:

```css
body {
    background: var(--bg);
    color: var(--fg);
}
```

### Terminal — swap themes at runtime

```rust
use palette_core::Registry;
use palette_core::terminal::to_terminal_theme;

let reg = Registry::new();
let theme = to_terminal_theme(&reg.load("tokyonight_storm")?);
```

### WASM

```js
import { JsRegistry } from "palette-core";

const reg = new JsRegistry();
const themes = reg.list(); // [{id, name, style}, ...]

const palette = reg.load("dracula");
```

## Developer-defined custom presets

Add your own presets — full themes or variants that inherit from a built-in.

**Variant that inherits from a built-in:**

```toml
# corporate_dark.toml
[meta]
name = "Corporate Dark"
preset_id = "corporate_dark"
schema_version = "1"
style = "dark"
kind = "preset-variant"
inherits = "tokyonight"

[semantic]
error = "#FF3333"
info = "#0099FF"
```

This theme gets all of tokyonight's colors, overriding only the semantic values.

**Full custom preset:**

```toml
# brand.toml
[meta]
name = "Brand Theme"
preset_id = "brand"
schema_version = "1"
style = "light"
kind = "preset-base"

[base]
background = "#FFFFFF"
foreground = "#1A1A1A"

[semantic]
error = "#CC0000"
success = "#008800"
```

**Register and use:**

```rust
use std::path::Path;
use palette_core::Registry;

let mut reg = Registry::new();

// Add a single file
reg.add_file(Path::new("themes/corporate_dark.toml"))?;

// Or add an entire directory of .toml files
reg.add_dir(Path::new("themes/"))?;

// Custom themes appear alongside built-ins
for info in reg.list() {
    println!("{}: {} ({})", info.id, info.name, info.style);
}

// Inheritance resolves automatically
let palette = reg.load("corporate_dark")?;
```

Custom variants can inherit from built-ins or from other custom presets already in the registry.

**WASM**

```js
import { JsRegistry } from "palette-core";

const reg = new JsRegistry();
reg.addToml(corporateDarkToml);

const palette = reg.load("corporate_dark");
```

## End-user-defined presets

Let your users load their own theme files at runtime. The same registry handles built-in, developer, and user themes.

```rust
use std::path::Path;
use palette_core::Registry;

let mut reg = Registry::new();

// Developer themes ship with the app
reg.add_dir(Path::new("themes/"))?;

// End-user themes loaded from a config directory
let user_themes_dir = dirs::config_dir()
    .map(|d| d.join("myapp/themes"));

if let Some(dir) = user_themes_dir.as_deref() {
    if dir.is_dir() {
        reg.add_dir(dir)?;
    }
}

// All themes — built-in, developer, and user — are in one list
for info in reg.list() {
    println!("{}: {} ({})", info.id, info.name, info.style);
}

let palette = reg.load(&user_selected_theme_id)?;
```

A user preset with the same `preset_id` as an existing theme replaces it, so users can override built-ins or developer themes.

User presets support inheritance — a user can write a variant that inherits from any theme already in the registry:

```toml
# ~/.config/myapp/themes/my_nord.toml
[meta]
name = "My Nord"
preset_id = "my_nord"
schema_version = "1"
style = "dark"
kind = "preset-variant"
inherits = "nord"

[base]
background = "#1a1a2e"
```

**WASM — user-supplied TOML string**

```js
const reg = new JsRegistry();
reg.addToml(userTomlString);
const palette = reg.load("my_nord");
```

## Contrast validation

Check foreground/background pairs against WCAG 2.1 contrast thresholds.

```rust
use palette_core::{preset, ContrastLevel};
use palette_core::contrast::validate_palette;

let palette = preset("tokyonight").expect("builtin preset");
let violations = validate_palette(&palette, ContrastLevel::AaNormal);
for v in &violations {
    println!("{} on {}: {:.2}:1", v.foreground_label, v.background_label, v.ratio);
}
```

Available levels: `AaNormal`, `AaLarge`, `AaaNormal`, `AaaLarge`.

### Auto-fix with `resolve_with_contrast`

`resolve_with_contrast` resolves a palette and nudges failing foreground colors until they meet the requested contrast level. Only HSL lightness is adjusted — hue and saturation are preserved. Background colors are never modified.

```rust
use palette_core::{preset, ContrastLevel};

let palette = preset("catppuccin").expect("builtin preset");
let resolved = palette.resolve_with_contrast(ContrastLevel::AaNormal);
// All foreground/background pairs now meet AA normal (≥ 4.5:1)
```

For individual color pairs, use `nudge_foreground` directly:

```rust
use palette_core::Color;
use palette_core::contrast::{nudge_foreground, ContrastLevel};

let fg = Color::from_hex("#777777").unwrap();
let bg = Color::from_hex("#808080").unwrap();
let fixed = nudge_foreground(fg, bg, ContrastLevel::AaNormal);
```

## Color manipulation

```rust
use palette_core::Color;

let base = Color::from_hex("#1a1b26")?;

let hover = base.lighten(0.1);
let disabled = base.desaturate(0.3);
let accent = base.rotate_hue(180.0);
let overlay = base.blend(Color::from_hex("#FF0000")?, 0.5);
let ratio = base.contrast_ratio(&Color::from_hex("#FFFFFF")?);
```

Methods: `lighten`, `darken`, `saturate`, `desaturate`, `rotate_hue`, `blend`, `contrast_ratio`, `meets_level`. Amounts are absolute (CSS color model). Non-finite inputs return the color unchanged.

## Platform overrides

Per-platform color overrides for themes that need different values on different targets. Requires the `platform` feature.

```rust
let manifest = palette_core::manifest::PaletteManifest::from_toml(toml_str)?;
let overrides = palette_core::platform::from_sections(&manifest.platform)?;
// overrides["terminal"].background, overrides["web"].foreground, etc.
```

## Preset format

Base presets define all sections. Variants declare `inherits` in `[meta]` and override only differing values.

```toml
[meta]
name = "My Theme Storm"
preset_id = "my_theme_storm"
schema_version = "1"
style = "storm"
kind = "preset-variant"
inherits = "my_theme"

[base]
background = "#24283b"
```

Sections: `base`, `semantic`, `diff`, `surface`, `typography`, `syntax`, `editor`, `terminal`, `syntax_style`.

## Feature flags

| Feature | Dependency | What it adds |
|---------|------------|--------------|
| `terminal` | `ratatui` | `Palette` → `ratatui::style::Color` maps |
| `egui` | `egui` | `Palette` → `egui::Visuals` |
| `snapshot` | `serde_json` | JSON serialization of `Palette` |
| `platform` | — | Parse `[platform.terminal]` / `[platform.web]` overrides |
| `wasm` | `wasm-bindgen`, `js-sys` | JavaScript bindings (includes `snapshot`) |
| `full` | all except `wasm` | `terminal` + `egui` + `snapshot` + `platform` |

Core functionality — parsing, merge, CSS export, WCAG contrast, color manipulation — requires no optional dependencies.

## Bundled presets

| Family | Presets |
|--------|--------|
| Ayu | `ayu_dark`, `ayu_light`, `ayu_mirage` |
| Catppuccin | `catppuccin`, `catppuccin_frappe`, `catppuccin_latte`, `catppuccin_macchiato` |
| Dracula | `dracula` |
| Golden Hour | `golden_hour`, `golden_hour_dusk`, `golden_hour_twilight` |
| Everforest | `everforest_dark`, `everforest_light` |
| GitHub | `github_dark`, `github_light` |
| Gruvbox | `gruvbox_dark`, `gruvbox_light` |
| Kanagawa | `kanagawa` |
| Monokai | `monokai` |
| Nord | `nord` |
| One | `one_dark`, `one_light` |
| Rosé Pine | `rose_pine`, `rose_pine_dawn`, `rose_pine_moon` |
| Solarized | `solarized_dark`, `solarized_light` |
| TokyoNight | `tokyonight`, `tokyonight_storm`, `tokyonight_day`, `tokyonight_moon` |

All presets are embedded at compile time via `include_str!`. Use `preset_ids()` to list them programmatically.
