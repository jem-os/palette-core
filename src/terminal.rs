//! Ratatui integration: convert a [`Palette`] into terminal-native colors.

use ratatui::style::Color as RatatuiColor;

use crate::color::Color;
use crate::palette::Palette;

/// Convert a [`Color`] to a ratatui RGB color.
pub fn to_ratatui_color(color: &Color) -> RatatuiColor {
    RatatuiColor::Rgb(color.r, color.g, color.b)
}

macro_rules! terminal_group {
    ($(#[$_meta:meta])* $color_type:ident { $($field:ident),+ $(,)? }) => {
        pastey::paste! {
            #[doc = concat!("Ratatui-native version of [`", stringify!($color_type), "`](crate::palette::", stringify!($color_type), ").")]
            #[derive(Debug, Clone)]
            pub struct [<Terminal $color_type>] {
                $(
                    #[doc = concat!("`", stringify!($field), "` slot.")]
                    pub $field: Option<RatatuiColor>,
                )+
            }

            impl [<Terminal $color_type>] {
                fn from_palette(group: &crate::palette::$color_type) -> Self {
                    Self {
                        $($field: group.$field.map(|c| to_ratatui_color(&c)),)+
                    }
                }
            }
        }
    };
}

crate::palette::color_fields!(terminal_group);

/// Complete ratatui-native theme mirroring every [`Palette`] color group.
#[derive(Debug, Clone)]
pub struct TerminalTheme {
    /// Core background and foreground colors.
    pub base: TerminalBaseColors,
    /// Status colors (success, warning, error, info, hint).
    pub semantic: TerminalSemanticColors,
    /// Version-control diff highlighting.
    pub diff: TerminalDiffColors,
    /// UI surface colors (menus, sidebars, overlays).
    pub surface: TerminalSurfaceColors,
    /// Text chrome (comments, line numbers, links).
    pub typography: TerminalTypographyColors,
    /// Syntax-highlighting token colors.
    pub syntax: TerminalSyntaxColors,
    /// Editor chrome (cursor, selections, diagnostics).
    pub editor: TerminalEditorColors,
    /// Standard 16-color ANSI terminal palette.
    pub terminal_ansi: TerminalTerminalAnsiColors,
}

/// Convert an entire [`Palette`] into a [`TerminalTheme`].
pub fn to_terminal_theme(palette: &Palette) -> TerminalTheme {
    TerminalTheme {
        base: TerminalBaseColors::from_palette(&palette.base),
        semantic: TerminalSemanticColors::from_palette(&palette.semantic),
        diff: TerminalDiffColors::from_palette(&palette.diff),
        surface: TerminalSurfaceColors::from_palette(&palette.surface),
        typography: TerminalTypographyColors::from_palette(&palette.typography),
        syntax: TerminalSyntaxColors::from_palette(&palette.syntax),
        editor: TerminalEditorColors::from_palette(&palette.editor),
        terminal_ansi: TerminalTerminalAnsiColors::from_palette(&palette.terminal_ansi),
    }
}
