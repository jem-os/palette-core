use crate::color::Color;
use crate::palette::Palette;
use crate::resolved::ResolvedPalette;

/// WCAG 2.1 conformance level for contrast checking.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContrastLevel {
    /// AA for normal text (≥ 4.5:1).
    AaNormal,
    /// AA for large text (≥ 3.0:1).
    AaLarge,
    /// AAA for normal text (≥ 7.0:1).
    AaaNormal,
    /// AAA for large text (≥ 4.5:1).
    AaaLarge,
}

impl ContrastLevel {
    /// Minimum contrast ratio required for this level.
    pub fn threshold(self) -> f64 {
        match self {
            Self::AaNormal | Self::AaaLarge => 4.5,
            Self::AaLarge => 3.0,
            Self::AaaNormal => 7.0,
        }
    }

    /// Whether the given ratio meets this conformance level.
    pub fn passes(self, ratio: f64) -> bool {
        ratio >= self.threshold()
    }
}

/// A foreground/background pair that fails a contrast check.
#[derive(Debug, Clone, PartialEq)]
pub struct ContrastViolation {
    /// Dot-path label of the foreground slot (e.g. `"base.foreground"`).
    pub foreground_label: Box<str>,
    /// Dot-path label of the background slot (e.g. `"base.background"`).
    pub background_label: Box<str>,
    /// The foreground color that was tested.
    pub foreground: Color,
    /// The background color that was tested.
    pub background: Color,
    /// Measured contrast ratio.
    pub ratio: f64,
    /// The conformance level that was not met.
    pub level: ContrastLevel,
}

/// WCAG 2.1 contrast ratio between two colors. Returns `[1.0, 21.0]`.
pub fn contrast_ratio(a: &Color, b: &Color) -> f64 {
    let la = a.relative_luminance();
    let lb = b.relative_luminance();
    let (lighter, darker) = match la >= lb {
        true => (la, lb),
        false => (lb, la),
    };
    (lighter + 0.05) / (darker + 0.05)
}

/// Whether `fg` over `bg` meets the given [`ContrastLevel`].
pub fn meets_level(fg: &Color, bg: &Color, level: ContrastLevel) -> bool {
    level.passes(contrast_ratio(fg, bg))
}

impl Color {
    /// WCAG 2.1 contrast ratio against another color.
    pub fn contrast_ratio(&self, other: &Color) -> f64 {
        contrast_ratio(self, other)
    }

    /// Whether contrast against `other` meets the given [`ContrastLevel`].
    pub fn meets_level(&self, other: &Color, level: ContrastLevel) -> bool {
        meets_level(self, other, level)
    }
}

fn check_pair(
    fg_prefix: &str,
    fg_name: &str,
    bg_prefix: &str,
    bg_name: &str,
    fg: Option<&Color>,
    bg: Option<&Color>,
    level: ContrastLevel,
) -> Option<ContrastViolation> {
    let (fg_color, bg_color) = match (fg, bg) {
        (Some(f), Some(b)) => (*f, *b),
        _ => return None,
    };
    let ratio = contrast_ratio(&fg_color, &bg_color);
    match level.passes(ratio) {
        true => None,
        false => Some(ContrastViolation {
            foreground_label: format!("{fg_prefix}.{fg_name}").into_boxed_str(),
            background_label: format!("{bg_prefix}.{bg_name}").into_boxed_str(),
            foreground: fg_color,
            background: bg_color,
            ratio,
            level,
        }),
    }
}

/// Check all semantically paired slots in a palette for contrast violations.
///
/// Returns an empty `Vec` when every tested pair meets the given level.
pub fn validate_palette(palette: &Palette, level: ContrastLevel) -> Vec<ContrastViolation> {
    let mut violations = Vec::new();
    let mut push = |v: Option<ContrastViolation>| {
        if let Some(v) = v {
            violations.push(v);
        }
    };

    // Core readability
    push(check_pair(
        "base",
        "foreground",
        "base",
        "background",
        palette.base.foreground.as_ref(),
        palette.base.background.as_ref(),
        level,
    ));
    push(check_pair(
        "base",
        "foreground_dark",
        "base",
        "background",
        palette.base.foreground_dark.as_ref(),
        palette.base.background.as_ref(),
        level,
    ));
    push(check_pair(
        "base",
        "foreground",
        "base",
        "background_dark",
        palette.base.foreground.as_ref(),
        palette.base.background_dark.as_ref(),
        level,
    ));
    push(check_pair(
        "base",
        "foreground",
        "base",
        "background_highlight",
        palette.base.foreground.as_ref(),
        palette.base.background_highlight.as_ref(),
        level,
    ));

    // Focus surface
    push(check_pair(
        "base",
        "foreground",
        "surface",
        "focus",
        palette.base.foreground.as_ref(),
        palette.surface.focus.as_ref(),
        level,
    ));

    // Semantic over background
    for (name, color) in palette.semantic.populated_slots() {
        push(check_pair(
            "semantic",
            name,
            "base",
            "background",
            Some(color),
            palette.base.background.as_ref(),
            level,
        ));
    }

    // Editor pairs
    push(check_pair(
        "editor",
        "selection_fg",
        "editor",
        "selection_bg",
        palette.editor.selection_fg.as_ref(),
        palette.editor.selection_bg.as_ref(),
        level,
    ));
    push(check_pair(
        "editor",
        "inlay_hint_fg",
        "editor",
        "inlay_hint_bg",
        palette.editor.inlay_hint_fg.as_ref(),
        palette.editor.inlay_hint_bg.as_ref(),
        level,
    ));
    push(check_pair(
        "editor",
        "search_fg",
        "editor",
        "search_bg",
        palette.editor.search_fg.as_ref(),
        palette.editor.search_bg.as_ref(),
        level,
    ));
    push(check_pair(
        "editor",
        "cursor_text",
        "editor",
        "cursor",
        palette.editor.cursor_text.as_ref(),
        palette.editor.cursor.as_ref(),
        level,
    ));

    // Diff pairs
    push(check_pair(
        "diff",
        "added_fg",
        "diff",
        "added_bg",
        palette.diff.added_fg.as_ref(),
        palette.diff.added_bg.as_ref(),
        level,
    ));
    push(check_pair(
        "diff",
        "modified_fg",
        "diff",
        "modified_bg",
        palette.diff.modified_fg.as_ref(),
        palette.diff.modified_bg.as_ref(),
        level,
    ));
    push(check_pair(
        "diff",
        "removed_fg",
        "diff",
        "removed_bg",
        palette.diff.removed_fg.as_ref(),
        palette.diff.removed_bg.as_ref(),
        level,
    ));

    // Typography over background
    push(check_pair(
        "typography",
        "comment",
        "base",
        "background",
        palette.typography.comment.as_ref(),
        palette.base.background.as_ref(),
        level,
    ));
    push(check_pair(
        "typography",
        "line_number",
        "base",
        "background",
        palette.typography.line_number.as_ref(),
        palette.base.background.as_ref(),
        level,
    ));

    // Syntax over background
    for (name, color) in palette.syntax.populated_slots() {
        push(check_pair(
            "syntax",
            name,
            "base",
            "background",
            Some(color),
            palette.base.background.as_ref(),
            level,
        ));
    }

    violations
}

/// Nudge a foreground color's lightness until it meets the given contrast level
/// against `bg`. Returns `fg` unchanged if the pair already passes or if no
/// lightness adjustment can reach the target.
///
/// Only HSL lightness is modified; hue and saturation are preserved.
pub fn nudge_foreground(fg: Color, bg: Color, level: ContrastLevel) -> Color {
    let threshold = level.threshold();
    if contrast_ratio(&fg, &bg) >= threshold {
        return fg;
    }

    let fg_lum = fg.relative_luminance();
    let bg_lum = bg.relative_luminance();

    // Try the natural direction first: lighter fg if fg is lighter, darker otherwise.
    let primary_lighten = fg_lum >= bg_lum;
    match nudge_direction(fg, &bg, threshold, primary_lighten) {
        Some(result) => result,
        None => nudge_direction(fg, &bg, threshold, !primary_lighten).unwrap_or(fg),
    }
}

fn nudge_direction(fg: Color, bg: &Color, threshold: f64, lighten: bool) -> Option<Color> {
    // Check if the extreme can reach the target at all.
    let extreme = match lighten {
        true => fg.lighten(1.0),
        false => fg.darken(1.0),
    };
    match contrast_ratio(&extreme, bg) >= threshold {
        true => {}
        false => return None,
    }

    let mut lo: f64 = 0.0;
    let mut hi: f64 = 1.0;
    let mut best = extreme;

    for _ in 0..20 {
        let mid = (lo + hi) / 2.0;
        let candidate = match lighten {
            true => fg.lighten(mid),
            false => fg.darken(mid),
        };
        match contrast_ratio(&candidate, bg) >= threshold {
            true => {
                best = candidate;
                hi = mid;
            }
            false => lo = mid,
        }
    }
    Some(best)
}

/// Adjust all semantically paired foreground slots on a resolved palette so
/// they meet the given contrast level. Mirrors the pairs checked by
/// [`validate_palette`].
pub fn adjust_contrast(resolved: &mut ResolvedPalette, level: ContrastLevel) {
    // Helper: nudge fg against bg in place.
    macro_rules! nudge {
        ($fg:expr, $bg:expr) => {
            $fg = nudge_foreground($fg, $bg, level);
        };
    }

    // Core readability
    nudge!(resolved.base.foreground, resolved.base.background);
    nudge!(resolved.base.foreground_dark, resolved.base.background);
    nudge!(resolved.base.foreground, resolved.base.background_dark);
    nudge!(resolved.base.foreground, resolved.base.background_highlight);

    // Focus surface
    nudge!(resolved.base.foreground, resolved.surface.focus);

    // Semantic over background
    nudge!(resolved.semantic.success, resolved.base.background);
    nudge!(resolved.semantic.warning, resolved.base.background);
    nudge!(resolved.semantic.error, resolved.base.background);
    nudge!(resolved.semantic.info, resolved.base.background);
    nudge!(resolved.semantic.hint, resolved.base.background);

    // Editor pairs
    nudge!(resolved.editor.selection_fg, resolved.editor.selection_bg);
    nudge!(resolved.editor.inlay_hint_fg, resolved.editor.inlay_hint_bg);
    nudge!(resolved.editor.search_fg, resolved.editor.search_bg);
    nudge!(resolved.editor.cursor_text, resolved.editor.cursor);

    // Diff pairs
    nudge!(resolved.diff.added_fg, resolved.diff.added_bg);
    nudge!(resolved.diff.modified_fg, resolved.diff.modified_bg);
    nudge!(resolved.diff.removed_fg, resolved.diff.removed_bg);

    // Typography over background
    nudge!(resolved.typography.comment, resolved.base.background);
    nudge!(resolved.typography.line_number, resolved.base.background);

    // Syntax over background
    let bg = resolved.base.background;
    for_each_syntax_slot(&mut resolved.syntax, |slot| {
        *slot = nudge_foreground(*slot, bg, level);
    });
}

/// Apply a closure to every mutable syntax color slot.
fn for_each_syntax_slot(
    syntax: &mut crate::resolved::ResolvedSyntaxColors,
    mut f: impl FnMut(&mut Color),
) {
    for (_, slot) in syntax.all_slots_mut() {
        f(slot);
    }
}
