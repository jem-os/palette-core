use std::io::Write;
use std::sync::Arc;

use palette_core::color::Color;
use palette_core::error::PaletteError;
use palette_core::registry::{load_preset, load_preset_file, preset, preset_ids};
use palette_core::{Registry, ThemeInfo};

#[test]
fn all_presets_load_with_background() {
    for id in preset_ids() {
        let palette = load_preset(id).unwrap_or_else(|e| panic!("preset {id} failed: {e}"));
        assert!(
            palette.base.background.is_some(),
            "preset {id} missing base.background"
        );
    }
}

#[test]
fn variant_inherits_parent_colors() {
    let storm = load_preset("tokyonight_storm").unwrap();

    assert_eq!(
        storm.base.background,
        Some(Color::from_hex("#24283b").unwrap()),
        "storm should use its own background"
    );
    assert_eq!(
        storm.semantic.success,
        Some(Color::from_hex("#73daca").unwrap()),
        "storm should inherit success from tokyonight base"
    );
}

#[test]
fn unknown_preset_returns_error() {
    let result = load_preset("nonexistent");
    assert!(matches!(result, Err(PaletteError::UnknownPreset(_))));
}

#[test]
fn preset_returns_palette_for_builtin() {
    let palette = preset("tokyonight").expect("builtin should exist");
    assert_eq!(
        palette.base.background,
        Some(Color::from_hex("#1a1b2a").unwrap()),
    );
}

#[test]
fn preset_returns_none_for_unknown_id() {
    assert!(preset("nonexistent").is_none());
}

#[test]
fn preset_resolves_inheritance() {
    let storm = preset("tokyonight_storm").expect("builtin variant should exist");
    assert_eq!(
        storm.base.background,
        Some(Color::from_hex("#24283b").unwrap()),
        "variant uses its own background"
    );
    assert_eq!(
        storm.semantic.success,
        Some(Color::from_hex("#73daca").unwrap()),
        "variant inherits from parent"
    );
}

#[test]
fn preset_ids_list_is_complete() {
    assert_eq!(preset_ids().len(), 31);
}

const MINIMAL_TOML: &str = r##"
[meta]
name = "Test Theme"
preset_id = "test_theme"
schema_version = "1"
style = "dark"
kind = "preset-base"

[base]
background = "#1a1b2a"
foreground = "#c0caf5"
"##;

const BASE_TOML: &str = r##"
[meta]
name = "Sibling Base"
preset_id = "sibling_base"
schema_version = "1"
style = "dark"
kind = "preset-base"

[base]
background = "#111111"
foreground = "#eeeeee"

[semantic]
success = "#00ff00"
"##;

const VARIANT_SIBLING_TOML: &str = r##"
[meta]
name = "Sibling Variant"
preset_id = "sibling_variant"
schema_version = "1"
style = "dark"
kind = "preset-variant"
inherits = "sibling_base"

[base]
background = "#222222"
"##;

const VARIANT_EMBEDDED_TOML: &str = r##"
[meta]
name = "Embedded Variant"
preset_id = "embedded_variant"
schema_version = "1"
style = "night"
kind = "preset-variant"
inherits = "tokyonight"

[base]
background = "#333333"
"##;

const VARIANT_MISSING_PARENT_TOML: &str = r##"
[meta]
name = "Orphan Variant"
preset_id = "orphan"
schema_version = "1"
style = "dark"
kind = "preset-variant"
inherits = "no_such_preset"

[base]
background = "#000000"
"##;

fn write_temp_file(dir: &tempfile::TempDir, name: &str, content: &str) -> std::path::PathBuf {
    let path = dir.path().join(name);
    let mut f = std::fs::File::create(&path).unwrap();
    f.write_all(content.as_bytes()).unwrap();
    path
}

#[test]
fn file_preset_loads_from_disk() {
    let dir = tempfile::tempdir().unwrap();
    let path = write_temp_file(&dir, "test_theme.toml", MINIMAL_TOML);

    let palette = load_preset_file(&path).unwrap();
    assert_eq!(
        palette.base.background,
        Some(Color::from_hex("#1a1b2a").unwrap()),
    );
}

#[test]
fn file_preset_inherits_from_sibling() {
    let dir = tempfile::tempdir().unwrap();
    write_temp_file(&dir, "sibling_base.toml", BASE_TOML);
    let variant_path = write_temp_file(&dir, "sibling_variant.toml", VARIANT_SIBLING_TOML);

    let palette = load_preset_file(&variant_path).unwrap();
    assert_eq!(
        palette.base.background,
        Some(Color::from_hex("#222222").unwrap()),
        "variant overrides base background"
    );
    assert_eq!(
        palette.base.foreground,
        Some(Color::from_hex("#eeeeee").unwrap()),
        "variant inherits foreground from sibling base"
    );
    assert_eq!(
        palette.semantic.success,
        Some(Color::from_hex("#00ff00").unwrap()),
        "variant inherits semantic.success from sibling base"
    );
}

#[test]
fn file_preset_inherits_from_embedded() {
    let dir = tempfile::tempdir().unwrap();
    let path = write_temp_file(&dir, "embedded_variant.toml", VARIANT_EMBEDDED_TOML);

    let palette = load_preset_file(&path).unwrap();
    assert_eq!(
        palette.base.background,
        Some(Color::from_hex("#333333").unwrap()),
        "variant uses its own background"
    );
    assert_eq!(
        palette.semantic.success,
        Some(Color::from_hex("#73daca").unwrap()),
        "variant inherits success from embedded tokyonight"
    );
}

#[test]
fn file_preset_missing_file_returns_error() {
    let result = load_preset_file(std::path::Path::new("/tmp/does_not_exist.toml"));
    assert!(matches!(result, Err(PaletteError::Io { .. })));
}

#[test]
fn file_preset_missing_parent_returns_error() {
    let dir = tempfile::tempdir().unwrap();
    let path = write_temp_file(&dir, "orphan.toml", VARIANT_MISSING_PARENT_TOML);

    let result = load_preset_file(&path);
    assert!(matches!(result, Err(PaletteError::UnknownPreset(_))));
}

// ---------------------------------------------------------------------------
// Registry tests
// ---------------------------------------------------------------------------

#[test]
fn registry_lists_all_builtins() {
    let reg = Registry::new();
    let themes: Vec<_> = reg.list().collect();
    assert_eq!(themes.len(), 31);
    for info in themes {
        assert!(!info.id.is_empty());
        assert!(!info.name.is_empty());
        assert!(!info.style.is_empty());
    }
}

#[test]
fn registry_load_builtin() {
    let reg = Registry::new();
    let palette = reg.load("tokyonight").unwrap();
    assert_eq!(
        palette.base.background,
        Some(Color::from_hex("#1a1b2a").unwrap()),
    );
}

#[test]
fn registry_load_unknown_returns_error() {
    let reg = Registry::new();
    let result = reg.load("nonexistent");
    assert!(matches!(result, Err(PaletteError::UnknownPreset(_))));
}

#[test]
fn registry_add_file_custom_theme() {
    let dir = tempfile::tempdir().unwrap();
    let path = write_temp_file(&dir, "test_theme.toml", MINIMAL_TOML);

    let mut reg = Registry::new();
    reg.add_file(&path).unwrap();

    assert_eq!(reg.list().count(), 32);

    let last = reg.list().last().unwrap();
    assert_eq!(last.id.as_ref(), "test_theme");
    assert_eq!(last.name.as_ref(), "Test Theme");
    assert_eq!(last.style.as_ref(), "dark");

    let palette = reg.load("test_theme").unwrap();
    assert_eq!(
        palette.base.background,
        Some(Color::from_hex("#1a1b2a").unwrap()),
    );
}

#[test]
fn registry_add_file_with_builtin_inheritance() {
    let dir = tempfile::tempdir().unwrap();
    let path = write_temp_file(&dir, "embedded_variant.toml", VARIANT_EMBEDDED_TOML);

    let mut reg = Registry::new();
    reg.add_file(&path).unwrap();

    let palette = reg.load("embedded_variant").unwrap();
    assert_eq!(
        palette.base.background,
        Some(Color::from_hex("#333333").unwrap()),
        "custom variant uses its own background"
    );
    assert_eq!(
        palette.semantic.success,
        Some(Color::from_hex("#73daca").unwrap()),
        "custom variant inherits success from builtin tokyonight"
    );
}

#[test]
fn registry_add_file_with_custom_inheritance() {
    let dir = tempfile::tempdir().unwrap();
    let base_path = write_temp_file(&dir, "sibling_base.toml", BASE_TOML);
    let variant_path = write_temp_file(&dir, "sibling_variant.toml", VARIANT_SIBLING_TOML);

    let mut reg = Registry::new();
    reg.add_file(&base_path).unwrap();
    reg.add_file(&variant_path).unwrap();

    let palette = reg.load("sibling_variant").unwrap();
    assert_eq!(
        palette.base.background,
        Some(Color::from_hex("#222222").unwrap()),
        "variant overrides background"
    );
    assert_eq!(
        palette.base.foreground,
        Some(Color::from_hex("#eeeeee").unwrap()),
        "variant inherits foreground from custom parent"
    );
    assert_eq!(
        palette.semantic.success,
        Some(Color::from_hex("#00ff00").unwrap()),
        "variant inherits success from custom parent"
    );
}

#[test]
fn registry_add_dir_loads_all_toml_files() {
    let dir = tempfile::tempdir().unwrap();
    write_temp_file(&dir, "test_theme.toml", MINIMAL_TOML);
    write_temp_file(&dir, "sibling_base.toml", BASE_TOML);
    write_temp_file(&dir, "not_toml.txt", "ignore me");

    let mut reg = Registry::new();
    reg.add_dir(dir.path()).unwrap();

    assert_eq!(reg.list().count(), 33);
}

#[test]
fn registry_duplicate_id_replaces_entry() {
    let replacement_toml = r##"
[meta]
name = "Custom Dracula"
preset_id = "dracula"
schema_version = "1"
style = "custom-dark"
kind = "preset-base"

[base]
background = "#aabbcc"
foreground = "#112233"
"##;

    let dir = tempfile::tempdir().unwrap();
    let path = write_temp_file(&dir, "dracula.toml", replacement_toml);

    let mut reg = Registry::new();
    reg.add_file(&path).unwrap();

    // Count should stay 31 (replaced, not appended)
    assert_eq!(reg.list().count(), 31);

    let dracula = reg.list().find(|t| t.id.as_ref() == "dracula").unwrap();
    assert_eq!(dracula.name.as_ref(), "Custom Dracula");
    assert_eq!(dracula.style.as_ref(), "custom-dark");

    let palette = reg.load("dracula").unwrap();
    assert_eq!(
        palette.base.background,
        Some(Color::from_hex("#aabbcc").unwrap()),
    );
}

#[test]
fn registry_by_style_returns_matching_themes() {
    let reg = Registry::new();
    let dark: Vec<_> = reg.by_style("dark").collect();
    assert!(!dark.is_empty());
    assert!(dark.iter().all(|t| t.style.as_ref() == "dark"));
}

#[test]
fn registry_by_style_nonexistent_returns_empty() {
    let reg = Registry::new();
    assert_eq!(reg.by_style("nonexistent").count(), 0);
}

#[test]
fn registry_add_toml_registers_custom_theme() {
    let mut reg = Registry::new();
    reg.add_toml(MINIMAL_TOML.to_owned()).unwrap();
    assert_eq!(reg.list().count(), 32);

    let last = reg.list().last().unwrap();
    assert_eq!(last.id.as_ref(), "test_theme");
}

#[test]
fn registry_builtin_metadata_matches_expected() {
    let reg = Registry::new();
    let tokyonight = reg.list().find(|t| t.id.as_ref() == "tokyonight").unwrap();
    assert_eq!(
        *tokyonight,
        ThemeInfo {
            id: Arc::from("tokyonight"),
            name: Arc::from("TokyoNight (Night)"),
            style: Arc::from("night"),
        }
    );
}
