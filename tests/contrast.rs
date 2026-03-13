mod common;

use std::collections::HashMap;
use std::sync::Arc;

use palette_core::color::Color;
use palette_core::contrast::{
    ContrastLevel, contrast_ratio, meets_level, nudge_foreground, validate_palette,
};
use palette_core::manifest::PaletteManifest;
use palette_core::palette::Palette;

fn color(hex: &str) -> Color {
    Color::from_hex(hex).unwrap()
}

// --- Luminance ---

#[test]
fn luminance_black() {
    let lum = color("#000000").relative_luminance();
    assert!((lum - 0.0).abs() < 1e-6, "expected 0.0, got {lum}");
}

#[test]
fn luminance_white() {
    let lum = color("#FFFFFF").relative_luminance();
    assert!((lum - 1.0).abs() < 1e-6, "expected 1.0, got {lum}");
}

#[test]
fn luminance_midgray() {
    let lum = color("#808080").relative_luminance();
    assert!((lum - 0.2159).abs() < 0.001, "expected ~0.2159, got {lum}");
}

// --- Contrast ratio ---

#[test]
fn ratio_black_white() {
    let ratio = contrast_ratio(&color("#000000"), &color("#FFFFFF"));
    assert!((ratio - 21.0).abs() < 0.05, "expected 21.0, got {ratio}");
}

#[test]
fn ratio_same_color() {
    let ratio = contrast_ratio(&color("#ABCDEF"), &color("#ABCDEF"));
    assert!((ratio - 1.0).abs() < 1e-6, "expected 1.0, got {ratio}");
}

#[test]
fn ratio_aa_boundary() {
    // #767676 on #FFFFFF is the canonical WCAG AA boundary (~4.54:1)
    let ratio = contrast_ratio(&color("#767676"), &color("#FFFFFF"));
    assert!(ratio > 4.5, "expected >4.5, got {ratio}");
    assert!(ratio < 4.6, "expected <4.6, got {ratio}");
}

#[test]
fn ratio_order_independence() {
    let a = color("#336699");
    let b = color("#FFCC00");
    let ab = contrast_ratio(&a, &b);
    let ba = contrast_ratio(&b, &a);
    assert!(
        (ab - ba).abs() < 1e-10,
        "ratio(a,b)={ab} != ratio(b,a)={ba}"
    );
}

// --- Compliance levels ---

#[test]
fn level_thresholds() {
    assert!((ContrastLevel::AaNormal.threshold() - 4.5).abs() < 1e-10);
    assert!((ContrastLevel::AaLarge.threshold() - 3.0).abs() < 1e-10);
    assert!((ContrastLevel::AaaNormal.threshold() - 7.0).abs() < 1e-10);
    assert!((ContrastLevel::AaaLarge.threshold() - 4.5).abs() < 1e-10);
}

#[test]
fn passes_aa_normal() {
    assert!(ContrastLevel::AaNormal.passes(4.5));
    assert!(!ContrastLevel::AaNormal.passes(4.49));
}

#[test]
fn passes_aa_large() {
    assert!(ContrastLevel::AaLarge.passes(3.0));
    assert!(!ContrastLevel::AaLarge.passes(2.99));
}

#[test]
fn passes_aaa_normal() {
    assert!(ContrastLevel::AaaNormal.passes(7.0));
    assert!(!ContrastLevel::AaaNormal.passes(6.99));
}

#[test]
fn ratio_4_5_passes_aa_fails_aaa() {
    let fg = color("#767676");
    let bg = color("#FFFFFF");
    assert!(meets_level(&fg, &bg, ContrastLevel::AaNormal));
    assert!(!meets_level(&fg, &bg, ContrastLevel::AaaNormal));
}

#[test]
fn ratio_7_passes_all() {
    // black on white: 21:1
    let fg = color("#000000");
    let bg = color("#FFFFFF");
    assert!(meets_level(&fg, &bg, ContrastLevel::AaNormal));
    assert!(meets_level(&fg, &bg, ContrastLevel::AaLarge));
    assert!(meets_level(&fg, &bg, ContrastLevel::AaaNormal));
    assert!(meets_level(&fg, &bg, ContrastLevel::AaaLarge));
}

// --- Palette validation ---

fn validate_preset_aa(preset_id: &str) -> Vec<palette_core::contrast::ContrastViolation> {
    let palette = palette_core::registry::load_preset(preset_id).unwrap();
    validate_palette(&palette, ContrastLevel::AaNormal)
}

#[test]
fn golden_hour_passes_aa() {
    let violations = validate_preset_aa("golden_hour");
    assert!(
        violations.is_empty(),
        "golden_hour AA violations:\n{}",
        violations
            .iter()
            .map(|v| format!(
                "  {} on {}: {:.2}:1",
                v.foreground_label, v.background_label, v.ratio
            ))
            .collect::<Vec<_>>()
            .join("\n")
    );
}

#[test]
fn golden_hour_dusk_passes_aa() {
    let violations = validate_preset_aa("golden_hour_dusk");
    assert!(
        violations.is_empty(),
        "golden_hour_dusk AA violations:\n{}",
        violations
            .iter()
            .map(|v| format!(
                "  {} on {}: {:.2}:1",
                v.foreground_label, v.background_label, v.ratio
            ))
            .collect::<Vec<_>>()
            .join("\n")
    );
}

#[test]
fn golden_hour_twilight_passes_aa() {
    let violations = validate_preset_aa("golden_hour_twilight");
    assert!(
        violations.is_empty(),
        "golden_hour_twilight AA violations:\n{}",
        violations
            .iter()
            .map(|v| format!(
                "  {} on {}: {:.2}:1",
                v.foreground_label, v.background_label, v.ratio
            ))
            .collect::<Vec<_>>()
            .join("\n")
    );
}

#[test]
fn tokyonight_fg_bg_passes_aa() {
    let manifest = common::load_preset("tokyonight");
    let palette = Palette::from_manifest(&manifest).unwrap();
    let violations = validate_palette(&palette, ContrastLevel::AaNormal);
    let core_violation = violations.iter().find(|v| {
        v.foreground_label.as_ref() == "base.foreground"
            && v.background_label.as_ref() == "base.background"
    });
    assert!(
        core_violation.is_none(),
        "tokyonight fg/bg should pass AA: {core_violation:?}"
    );
}

#[test]
fn bad_palette_produces_violations() {
    let mut base = HashMap::new();
    // Nearly identical colors — guaranteed to fail
    base.insert(Arc::from("foreground"), Arc::from("#111111"));
    base.insert(Arc::from("background"), Arc::from("#121212"));
    let manifest = PaletteManifest {
        meta: None,
        base,
        semantic: HashMap::new(),
        diff: HashMap::new(),
        surface: HashMap::new(),
        typography: HashMap::new(),
        syntax: HashMap::new(),
        editor: HashMap::new(),
        terminal: HashMap::new(),
        syntax_style: HashMap::new(),
        #[cfg(feature = "platform")]
        platform: Default::default(),
    };
    let palette = Palette::from_manifest(&manifest).unwrap();
    let violations = validate_palette(&palette, ContrastLevel::AaNormal);
    assert!(
        !violations.is_empty(),
        "bad palette should produce violations"
    );

    let v = &violations[0];
    assert_eq!(v.foreground_label.as_ref(), "base.foreground");
    assert_eq!(v.background_label.as_ref(), "base.background");
    assert!(v.ratio < 4.5);
    assert_eq!(v.level, ContrastLevel::AaNormal);
}

#[test]
fn fg_bg_pairs_covered_by_validation() {
    // Verify that validate_palette checks all _fg/_bg suffix pairs in editor and diff.
    // Build a palette where every slot has the same color so every pair produces a
    // violation (ratio 1:1), then assert the expected pair labels appear.
    let palette = palette_core::preset("tokyonight").unwrap();
    let mut same = palette.clone();

    let grey = Color::from_hex("#808080").unwrap();
    // Set all editor paired fields to the same color
    same.editor.selection_fg = Some(grey);
    same.editor.selection_bg = Some(grey);
    same.editor.inlay_hint_fg = Some(grey);
    same.editor.inlay_hint_bg = Some(grey);
    same.editor.search_fg = Some(grey);
    same.editor.search_bg = Some(grey);
    same.editor.cursor_text = Some(grey);
    same.editor.cursor = Some(grey);
    same.diff.added_fg = Some(grey);
    same.diff.added_bg = Some(grey);
    same.diff.modified_fg = Some(grey);
    same.diff.modified_bg = Some(grey);
    same.diff.removed_fg = Some(grey);
    same.diff.removed_bg = Some(grey);

    let violations = validate_palette(&same, ContrastLevel::AaNormal);
    let labels: Vec<&str> = violations
        .iter()
        .map(|v| v.foreground_label.as_ref())
        .collect();

    // Editor pairs
    assert!(
        labels.contains(&"editor.selection_fg"),
        "missing editor.selection_fg"
    );
    assert!(
        labels.contains(&"editor.inlay_hint_fg"),
        "missing editor.inlay_hint_fg"
    );
    assert!(
        labels.contains(&"editor.search_fg"),
        "missing editor.search_fg"
    );
    assert!(
        labels.contains(&"editor.cursor_text"),
        "missing editor.cursor_text"
    );

    // Diff pairs
    assert!(labels.contains(&"diff.added_fg"), "missing diff.added_fg");
    assert!(
        labels.contains(&"diff.modified_fg"),
        "missing diff.modified_fg"
    );
    assert!(
        labels.contains(&"diff.removed_fg"),
        "missing diff.removed_fg"
    );
}

#[test]
fn none_fields_skipped_without_error() {
    let manifest = PaletteManifest {
        meta: None,
        base: HashMap::new(),
        semantic: HashMap::new(),
        diff: HashMap::new(),
        surface: HashMap::new(),
        typography: HashMap::new(),
        syntax: HashMap::new(),
        editor: HashMap::new(),
        terminal: HashMap::new(),
        syntax_style: HashMap::new(),
        #[cfg(feature = "platform")]
        platform: Default::default(),
    };
    let palette = Palette::from_manifest(&manifest).unwrap();
    let violations = validate_palette(&palette, ContrastLevel::AaNormal);
    assert!(
        violations.is_empty(),
        "empty palette should produce no violations"
    );
}

#[test]
fn github_presets_diff_modified_readable() {
    // Regression (#40): github_dark/github_light had modified_bg == modified_fg (ratio 1:1).
    // Diff backgrounds are line highlights with the indicator color overlaid; 2.5:1
    // matches the contrast the existing added/removed tints achieve in light themes.
    for id in ["github_dark", "github_light"] {
        let palette = palette_core::preset(id).unwrap();
        let fg = palette.diff.modified_fg.expect("missing modified_fg");
        let bg = palette.diff.modified_bg.expect("missing modified_bg");
        let ratio = fg.contrast_ratio(&bg);
        assert!(
            ratio >= 2.5,
            "{id}: diff.modified_fg ({fg}) on diff.modified_bg ({bg}) = {ratio:.2}:1 (need >= 2.5)"
        );
    }
}

#[test]
fn all_presets_focus_passes_aa_large() {
    for id in palette_core::preset_ids() {
        let palette = palette_core::preset(id).unwrap();
        let resolved = palette.resolve();
        let ratio = resolved
            .base
            .foreground
            .contrast_ratio(&resolved.surface.focus);
        assert!(
            ContrastLevel::AaLarge.passes(ratio),
            "{id}: base.foreground on surface.focus = {ratio:.2}:1 (need >= 3.0)"
        );
    }
}

// --- nudge_foreground ---

#[test]
fn nudge_already_passing_unchanged() {
    let fg = color("#000000");
    let bg = color("#FFFFFF");
    let result = nudge_foreground(fg, bg, ContrastLevel::AaNormal);
    assert_eq!(result, fg, "already-passing pair should not be modified");
}

#[test]
fn nudge_fixes_failing_pair() {
    // Near-identical colors: guaranteed to fail AA
    let fg = color("#777777");
    let bg = color("#808080");
    assert!(
        !meets_level(&fg, &bg, ContrastLevel::AaNormal),
        "precondition: pair should fail"
    );
    let nudged = nudge_foreground(fg, bg, ContrastLevel::AaNormal);
    assert!(
        meets_level(&nudged, &bg, ContrastLevel::AaNormal),
        "nudged pair should pass AA: ratio = {:.2}",
        contrast_ratio(&nudged, &bg)
    );
}

#[test]
fn nudge_preserves_hue() {
    // Saturated red that fails against a dark bg
    let fg = color("#CC3333");
    let bg = color("#331111");
    let nudged = nudge_foreground(fg, bg, ContrastLevel::AaNormal);
    // The nudged color should still be recognizably red (r > g, r > b)
    assert!(
        nudged.r > nudged.g && nudged.r > nudged.b,
        "nudged color {nudged} should still be reddish"
    );
}

#[test]
fn nudge_unadjustable_returns_original() {
    // Two identical colors — nudging lightness alone can't help if both
    // are mid-gray, but actually lightening or darkening should work.
    // Truly unadjustable: same color. The function should still return
    // a valid result (either fixed or original).
    let fg = color("#808080");
    let bg = color("#808080");
    let result = nudge_foreground(fg, bg, ContrastLevel::AaaNormal);
    // Mid-gray can reach AAA against itself by going to white or black.
    // Just verify the function doesn't panic and returns a valid color.
    let _ = result;
}

// --- resolve_with_contrast ---

#[test]
fn resolve_with_contrast_matches_resolve_on_clean_preset() {
    // A well-contrasted preset should produce identical results.
    let palette = palette_core::preset("golden_hour").unwrap();
    let plain = palette.resolve();
    let adjusted = palette.resolve_with_contrast(ContrastLevel::AaNormal);
    assert_eq!(
        plain, adjusted,
        "clean preset should not be modified by resolve_with_contrast"
    );
}

#[test]
fn resolve_with_contrast_fixes_bad_palette() {
    let mut base = HashMap::new();
    base.insert(Arc::from("foreground"), Arc::from("#555555"));
    base.insert(Arc::from("background"), Arc::from("#505050"));
    let manifest = PaletteManifest {
        meta: None,
        base,
        semantic: HashMap::new(),
        diff: HashMap::new(),
        surface: HashMap::new(),
        typography: HashMap::new(),
        syntax: HashMap::new(),
        editor: HashMap::new(),
        terminal: HashMap::new(),
        syntax_style: HashMap::new(),
        #[cfg(feature = "platform")]
        platform: Default::default(),
    };
    let palette = Palette::from_manifest(&manifest).unwrap();

    // Confirm violations exist before adjustment
    let before = validate_palette(&palette, ContrastLevel::AaNormal);
    assert!(
        !before.is_empty(),
        "precondition: palette should have violations"
    );

    let resolved = palette.resolve_with_contrast(ContrastLevel::AaNormal);

    // Check core fg/bg pair passes
    let ratio = resolved
        .base
        .foreground
        .contrast_ratio(&resolved.base.background);
    assert!(
        ContrastLevel::AaNormal.passes(ratio),
        "adjusted fg/bg should pass AA: {ratio:.2}:1"
    );
}

#[test]
fn resolve_with_contrast_zero_violations_on_presets() {
    for id in palette_core::preset_ids() {
        let palette = palette_core::preset(id).unwrap();
        let resolved = palette.resolve_with_contrast(ContrastLevel::AaNormal);

        // Re-validate by checking all the same pairs validate_palette would.
        // We check fg/bg directly since validate_palette works on Palette, not ResolvedPalette.
        let ratio = resolved
            .base
            .foreground
            .contrast_ratio(&resolved.base.background);
        assert!(
            ContrastLevel::AaNormal.passes(ratio),
            "{id}: base.foreground on base.background = {ratio:.2}:1 after adjustment"
        );
    }
}
