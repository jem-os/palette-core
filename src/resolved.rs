//! Resolved palette types with concrete [`Color`] fields.
//!
//! A [`ResolvedPalette`] mirrors [`Palette`](crate::Palette) but every color
//! slot is a bare [`Color`] instead of `Option<Color>`. Obtain one by calling
//! [`Palette::resolve`] (fills gaps from [`Palette::default`]) or
//! [`Palette::resolve_with`] (fills gaps from a custom fallback).

use std::sync::LazyLock;

use crate::color::Color;
use crate::contrast::{ContrastLevel, adjust_contrast};
use crate::palette::Palette;
use crate::style::ResolvedSyntaxStyles;

static DEFAULT_PALETTE: LazyLock<Palette> = LazyLock::new(Palette::default);

macro_rules! resolved_group {
    ($(#[$_meta:meta])* $color_type:ident { $($field:ident),+ $(,)? }) => {
        pastey::paste! {
            #[doc = concat!("Resolved version of [`", stringify!($color_type), "`](crate::palette::", stringify!($color_type), ") with concrete [`Color`] fields.")]
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            #[cfg_attr(feature = "snapshot", derive(serde::Serialize))]
            pub struct [<Resolved $color_type>] {
                $(
                    #[doc = concat!("`", stringify!($field), "` slot.")]
                    pub $field: Color,
                )+
            }

            impl [<Resolved $color_type>] {
                /// Build from an optional-field group, filling gaps with [`Color::default`] (black).
                ///
                /// Use [`Palette::resolve`] with the complete default palette to
                /// avoid black fallbacks. Black only appears when using
                /// [`Palette::resolve_with`] against an incomplete custom fallback.
                pub fn from_group(group: &crate::palette::$color_type) -> Self {
                    Self {
                        $($field: group.$field.unwrap_or_default(),)+
                    }
                }

                /// Iterate over all slots as `(name, &Color)` pairs.
                pub fn all_slots(&self) -> impl Iterator<Item = (&'static str, &Color)> {
                    [$(
                        (stringify!($field), &self.$field),
                    )+]
                    .into_iter()
                }

                /// Iterate over all slots as `(name, &mut Color)` pairs.
                pub fn all_slots_mut(&mut self) -> impl Iterator<Item = (&'static str, &mut Color)> {
                    [$(
                        (stringify!($field), &mut self.$field),
                    )+]
                    .into_iter()
                }
            }
        }
    };
}

crate::palette::color_fields!(resolved_group);

impl ResolvedSyntaxColors {
    /// Build from an optional-field group, resolving sub-token slots through
    /// their fallback parent before falling back to [`Color::default`].
    ///
    /// For the 12 sub-token slots (e.g. `keywords_control`), resolution order
    /// is: `self.slot → self.parent → Color::default()`.
    pub fn from_group_with_fallback(group: &crate::palette::SyntaxColors) -> Self {
        let mut resolved = Self::from_group(group);
        macro_rules! apply_fallback {
            ($($child:ident => $parent:ident),+ $(,)?) => {
                $(resolved.$child = group.$child.or(group.$parent).unwrap_or_default();)+
            };
        }
        crate::palette::syntax_fallback!(apply_fallback);
        resolved
    }
}

/// Fully resolved palette where every color slot is a concrete [`Color`].
///
/// Built via [`Palette::resolve`] or [`Palette::resolve_with`].
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "snapshot", derive(serde::Serialize))]
pub struct ResolvedPalette {
    /// Theme identity, if the source palette had metadata.
    pub meta: Option<crate::palette::PaletteMeta>,
    /// Core background and foreground colors.
    pub base: ResolvedBaseColors,
    /// Status colors (success, warning, error, info, hint).
    pub semantic: ResolvedSemanticColors,
    /// Version-control diff highlighting.
    pub diff: ResolvedDiffColors,
    /// UI surface colors (menus, sidebars, overlays).
    pub surface: ResolvedSurfaceColors,
    /// Text chrome (comments, line numbers, links).
    pub typography: ResolvedTypographyColors,
    /// Syntax-highlighting token colors.
    pub syntax: ResolvedSyntaxColors,
    /// Editor chrome (cursor, selections, diagnostics).
    pub editor: ResolvedEditorColors,
    /// Standard 16-color ANSI terminal palette.
    pub terminal_ansi: ResolvedTerminalAnsiColors,
    /// Syntax token style modifiers.
    pub syntax_style: ResolvedSyntaxStyles,
}

impl Palette {
    /// Resolve all `Option<Color>` slots using [`Palette::default`] as fallback.
    pub fn resolve(&self) -> ResolvedPalette {
        self.resolve_with(&DEFAULT_PALETTE)
    }

    /// Resolve all slots and nudge foreground colors to meet the given
    /// [`ContrastLevel`]. The TOML-defined colors remain authoritative;
    /// only failing foreground slots are lightened or darkened.
    pub fn resolve_with_contrast(&self, level: ContrastLevel) -> ResolvedPalette {
        let mut resolved = self.resolve();
        adjust_contrast(&mut resolved, level);
        resolved
    }

    /// Resolve all `Option<Color>` slots using a custom fallback palette.
    ///
    /// Slots absent in both `self` and `fallback` resolve to
    /// [`Color::default`] (black). Use [`resolve`](Self::resolve) with the
    /// complete default palette to avoid this.
    pub fn resolve_with(&self, fallback: &Palette) -> ResolvedPalette {
        ResolvedPalette {
            meta: self.meta.clone(),
            base: ResolvedBaseColors::from_group(&self.base.merge(&fallback.base)),
            semantic: ResolvedSemanticColors::from_group(&self.semantic.merge(&fallback.semantic)),
            diff: ResolvedDiffColors::from_group(&self.diff.merge(&fallback.diff)),
            surface: ResolvedSurfaceColors::from_group(&self.surface.merge(&fallback.surface)),
            typography: ResolvedTypographyColors::from_group(
                &self.typography.merge(&fallback.typography),
            ),
            syntax: ResolvedSyntaxColors::from_group_with_fallback(
                &self.syntax.merge(&fallback.syntax),
            ),
            editor: ResolvedEditorColors::from_group(&self.editor.merge(&fallback.editor)),
            terminal_ansi: ResolvedTerminalAnsiColors::from_group(
                &self.terminal_ansi.merge(&fallback.terminal_ansi),
            ),
            syntax_style: ResolvedSyntaxStyles::from_group_with_fallback(
                &self.syntax_style.merge(&fallback.syntax_style),
            ),
        }
    }
}
