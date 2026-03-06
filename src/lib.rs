//! TOML-defined theme system with inheritance and multi-target export.
//!
//! palette-core parses theme files written in TOML, resolves single-level
//! inheritance between variants, and produces a [`Palette`] — a typed color
//! map covering base, semantic, syntax, editor, diff, surface, typography,
//! and terminal-ANSI slots.
//!
//! # Quick start
//!
//! ```
//! use palette_core::{preset, load_preset, Registry};
//!
//! // Infallible load of a built-in theme
//! let palette = preset("catppuccin").unwrap();
//!
//! // Fallible load (returns PaletteError on unknown ID)
//! let palette = load_preset("tokyonight").unwrap();
//!
//! // Registry: built-ins + custom themes from disk
//! let mut reg = Registry::new();
//! reg.add_dir("./my-themes".as_ref()).ok();
//! for info in reg.list() {
//!     println!("{}: {}", info.id, info.name);
//! }
//! ```
//!
//! # Export targets
//!
//! | Target | Feature | Function |
//! |--------|---------|----------|
//! | CSS custom properties | — | [`Palette::to_css`](css) |
//! | JSON snapshot | `snapshot` | [`Palette::to_json`](snapshot) |
//! | ratatui `Color` | `terminal` | [`terminal::to_terminal_theme`] |
//! | egui `Visuals` | `egui` | [`egui::to_egui_visuals`] |
//! | WASM/JS bindings | `wasm` | `wasm` module |

/// 8-bit RGB color type and hex parsing.
pub mod color;
/// Error types for theme loading and parsing.
pub mod error;
/// Raw TOML manifest types before color resolution.
pub mod manifest;
/// Single-level manifest inheritance (variant over base).
pub mod merge;
/// Resolved color palette and color-group structs.
pub mod palette;
/// Built-in preset registry and theme discovery.
pub mod registry;

/// WCAG 2.1 contrast ratio checking and palette validation.
pub mod contrast;
/// CSS custom-property export.
pub mod css;
/// HSL color manipulation: lighten, darken, saturate, blend.
pub mod manipulation;

pub use color::Color;
pub use contrast::ContrastLevel;
pub use error::PaletteError;
pub use palette::{Palette, PaletteMeta};
pub use registry::{Registry, ThemeInfo, load_preset, load_preset_file, preset, preset_ids};

#[cfg(feature = "terminal")]
pub mod terminal;

#[cfg(feature = "platform")]
pub mod platform;

#[cfg(feature = "snapshot")]
pub mod snapshot;

#[cfg(feature = "egui")]
pub mod egui;

#[cfg(feature = "wasm")]
pub mod wasm;
