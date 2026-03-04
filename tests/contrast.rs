mod common;

use std::collections::BTreeMap;
use std::sync::Arc;

use palette_core::color::Color;
use palette_core::contrast::{ContrastLevel, contrast_ratio, meets_level, validate_palette};
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
    let mut base = BTreeMap::new();
    // Nearly identical colors — guaranteed to fail
    base.insert(Arc::from("foreground"), Arc::from("#111111"));
    base.insert(Arc::from("background"), Arc::from("#121212"));
    let manifest = PaletteManifest {
        meta: None,
        base,
        semantic: BTreeMap::new(),
        diff: BTreeMap::new(),
        surface: BTreeMap::new(),
        typography: BTreeMap::new(),
        syntax: BTreeMap::new(),
        editor: BTreeMap::new(),
        terminal: BTreeMap::new(),
        #[cfg(feature = "platform")]
        platform: BTreeMap::new(),
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
fn none_fields_skipped_without_error() {
    let manifest = PaletteManifest {
        meta: None,
        base: BTreeMap::new(),
        semantic: BTreeMap::new(),
        diff: BTreeMap::new(),
        surface: BTreeMap::new(),
        typography: BTreeMap::new(),
        syntax: BTreeMap::new(),
        editor: BTreeMap::new(),
        terminal: BTreeMap::new(),
        #[cfg(feature = "platform")]
        platform: BTreeMap::new(),
    };
    let palette = Palette::from_manifest(&manifest).unwrap();
    let violations = validate_palette(&palette, ContrastLevel::AaNormal);
    assert!(
        violations.is_empty(),
        "empty palette should produce no violations"
    );
}
