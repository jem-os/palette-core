use crate::color::Color;
use crate::palette::Palette;

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
