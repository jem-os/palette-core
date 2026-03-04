use std::path::Path;
use std::sync::Arc;

use crate::error::PaletteError;
use crate::manifest::PaletteManifest;
use crate::merge::merge_manifests;
use crate::palette::Palette;

/// Display metadata for a theme, usable without parsing the full TOML.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ThemeInfo {
    pub id: Arc<str>,
    pub name: Arc<str>,
    pub style: Arc<str>,
}

struct BuiltinInfo {
    id: &'static str,
    name: &'static str,
    style: &'static str,
}

macro_rules! presets {
    ($($id:literal => $file:literal, $name:literal, $style:literal),+ $(,)?) => {
        pub(crate) fn preset_toml(id: &str) -> Option<&'static str> {
            match id {
                $($id => Some(include_str!($file)),)+
                _ => None,
            }
        }

        /// All built-in preset IDs, in declaration order.
        pub fn preset_ids() -> &'static [&'static str] {
            &[$($id),+]
        }

        fn builtin_info() -> &'static [BuiltinInfo] {
            &[$(BuiltinInfo {
                id: $id,
                name: $name,
                style: $style,
            }),+]
        }
    };
}

presets! {
    "ayu_dark"              => "../presets/ayu_dark.toml",              "Ayu Dark",              "dark",
    "ayu_light"             => "../presets/ayu_light.toml",             "Ayu Light",             "light",
    "ayu_mirage"            => "../presets/ayu_mirage.toml",            "Ayu Mirage",            "mirage",
    "catppuccin"            => "../presets/catppuccin.toml",            "Catppuccin Mocha",      "mocha",
    "catppuccin_frappe"     => "../presets/catppuccin_frappe.toml",     "Catppuccin Frappe",     "frappe",
    "catppuccin_latte"      => "../presets/catppuccin_latte.toml",      "Catppuccin Latte",      "latte",
    "catppuccin_macchiato"  => "../presets/catppuccin_macchiato.toml",  "Catppuccin Macchiato",  "macchiato",
    "dracula"               => "../presets/dracula.toml",               "Dracula",               "dark",
    "everforest_dark"       => "../presets/everforest_dark.toml",       "Everforest Dark",       "dark",
    "everforest_light"      => "../presets/everforest_light.toml",      "Everforest Light",      "light",
    "github_dark"           => "../presets/github_dark.toml",           "GitHub Dark",           "dark",
    "github_light"          => "../presets/github_light.toml",          "GitHub Light",          "light",
    "golden_hour"           => "../presets/golden_hour.toml",           "Golden Hour",           "light",
    "golden_hour_dusk"      => "../presets/golden_hour_dusk.toml",      "Golden Hour (Dusk)",    "dark",
    "golden_hour_twilight"  => "../presets/golden_hour_twilight.toml",  "Golden Hour (Twilight)","dark",
    "gruvbox_dark"          => "../presets/gruvbox_dark.toml",          "Gruvbox Dark",          "dark",
    "gruvbox_light"         => "../presets/gruvbox_light.toml",         "Gruvbox Light",         "light",
    "kanagawa"              => "../presets/kanagawa.toml",              "Kanagawa",              "dark",
    "monokai"               => "../presets/monokai.toml",               "Monokai",               "dark",
    "nord"                  => "../presets/nord.toml",                  "Nord",                  "dark",
    "one_dark"              => "../presets/one_dark.toml",              "One Dark",              "dark",
    "one_light"             => "../presets/one_light.toml",             "One Light",             "light",
    "rose_pine"             => "../presets/rose_pine.toml",             "Rose Pine",             "dark",
    "rose_pine_dawn"        => "../presets/rose_pine_dawn.toml",        "Rose Pine Dawn",        "dawn",
    "rose_pine_moon"        => "../presets/rose_pine_moon.toml",        "Rose Pine Moon",        "moon",
    "solarized_dark"        => "../presets/solarized_dark.toml",        "Solarized Dark",        "dark",
    "solarized_light"       => "../presets/solarized_light.toml",       "Solarized Light",       "light",
    "tokyonight"            => "../presets/tokyonight.toml",            "TokyoNight (Night)",    "night",
    "tokyonight_day"        => "../presets/tokyonight_day.toml",        "TokyoNight Day",        "day",
    "tokyonight_moon"       => "../presets/tokyonight_moon.toml",       "TokyoNight Moon",       "moon",
    "tokyonight_storm"      => "../presets/tokyonight_storm.toml",      "TokyoNight Storm",      "storm",
}

// ---------------------------------------------------------------------------
// Shared inheritance resolution
// ---------------------------------------------------------------------------

fn resolve_with_inheritance<F>(toml_str: &str, resolve_parent: F) -> Result<Palette, PaletteError>
where
    F: FnOnce(&str) -> Result<PaletteManifest, PaletteError>,
{
    let manifest = PaletteManifest::from_toml(toml_str)?;
    let resolved = match manifest.inherits_from() {
        None => manifest,
        Some(parent_id) => {
            let parent = resolve_parent(parent_id)?;
            merge_manifests(&manifest, &parent)
        }
    };
    Palette::from_manifest(&resolved)
}

// ---------------------------------------------------------------------------
// Standalone preset functions (existing API)
// ---------------------------------------------------------------------------

/// Load a theme from a TOML file on disk, resolving inheritance from sibling
/// files or built-in presets.
pub fn load_preset_file(path: &Path) -> Result<Palette, PaletteError> {
    let path_str: Arc<str> = Arc::from(path.to_string_lossy().as_ref());
    let toml = std::fs::read_to_string(path).map_err(|source| PaletteError::Io {
        path: path_str,
        source,
    })?;
    resolve_with_inheritance(&toml, |parent_id| resolve_parent(path, parent_id))
}

fn resolve_parent(child_path: &Path, parent_id: &str) -> Result<PaletteManifest, PaletteError> {
    let sibling = child_path
        .parent()
        .map(|dir| dir.join(format!("{parent_id}.toml")))
        .filter(|p| p.is_file());

    match (sibling, preset_toml(parent_id)) {
        (Some(path), _) => {
            let path_str: Arc<str> = Arc::from(path.to_string_lossy().as_ref());
            let toml = std::fs::read_to_string(&path).map_err(|source| PaletteError::Io {
                path: path_str,
                source,
            })?;
            PaletteManifest::from_toml(&toml)
        }
        (None, Some(embedded)) => PaletteManifest::from_toml(embedded),
        (None, None) => Err(PaletteError::UnknownPreset(Arc::from(parent_id))),
    }
}

/// Load a built-in preset by ID, resolving inheritance.
///
/// Returns [`PaletteError::UnknownPreset`] if the ID is not recognized.
/// For an infallible alternative, see [`preset`].
pub fn load_preset(id: &str) -> Result<Palette, PaletteError> {
    let toml = preset_toml(id).ok_or_else(|| PaletteError::UnknownPreset(Arc::from(id)))?;
    resolve_with_inheritance(toml, |parent_id| {
        let parent_toml = preset_toml(parent_id)
            .ok_or_else(|| PaletteError::UnknownPreset(Arc::from(parent_id)))?;
        PaletteManifest::from_toml(parent_toml)
    })
}

/// Load a built-in preset by ID, returning `None` if the ID is not recognized.
///
/// Built-in presets are compiled into the binary and test-verified, so parsing
/// cannot fail. This provides an infallible alternative to [`load_preset`] for
/// the common case of loading a known built-in theme.
pub fn preset(id: &str) -> Option<Palette> {
    load_preset(id).ok()
}

// ---------------------------------------------------------------------------
// Registry
// ---------------------------------------------------------------------------

enum Source {
    Builtin,
    Custom(Box<str>),
}

struct Entry {
    info: ThemeInfo,
    source: Source,
}

/// Unified theme registry combining built-in presets with custom themes.
///
/// Built-in themes carry static metadata (name, style) without parsing TOML.
/// Custom themes are added via files or directories and stored as raw TOML.
pub struct Registry {
    entries: Vec<Entry>,
}

impl Registry {
    /// Create a registry pre-populated with all built-in presets.
    pub fn new() -> Self {
        let entries: Vec<Entry> = builtin_info()
            .iter()
            .map(|b| Entry {
                info: ThemeInfo {
                    id: Arc::from(b.id),
                    name: Arc::from(b.name),
                    style: Arc::from(b.style),
                },
                source: Source::Builtin,
            })
            .collect();
        Self { entries }
    }

    /// All registered themes in insertion order (built-ins first, then custom).
    pub fn list(&self) -> impl Iterator<Item = &ThemeInfo> {
        self.entries.iter().map(|e| &e.info)
    }

    /// Load a palette by ID, resolving inheritance within the registry.
    pub fn load(&self, id: &str) -> Result<Palette, PaletteError> {
        let toml_str = self.toml_for(id)?;
        resolve_with_inheritance(toml_str, |parent_id| self.resolve_manifest(parent_id))
    }

    /// Filter registered themes by style (e.g. "dark", "light").
    pub fn by_style(&self, style: &str) -> impl Iterator<Item = &ThemeInfo> {
        self.entries
            .iter()
            .filter(move |e| e.info.style.as_ref() == style)
            .map(|e| &e.info)
    }

    /// Register a custom theme from a TOML file on disk.
    pub fn add_file(&mut self, path: &Path) -> Result<(), PaletteError> {
        let path_str: Arc<str> = Arc::from(path.to_string_lossy().as_ref());
        let toml = std::fs::read_to_string(path).map_err(|source| PaletteError::Io {
            path: path_str,
            source,
        })?;

        self.add_toml(toml)
    }

    /// Register a custom theme from a TOML string.
    ///
    /// Useful for WASM targets (no filesystem), network-fetched themes, or
    /// embedded resources.
    pub fn add_toml(&mut self, toml: String) -> Result<(), PaletteError> {
        let info = extract_theme_info(&toml)?;
        let source = Source::Custom(toml.into_boxed_str());

        match self.entries.iter().position(|e| e.info.id == info.id) {
            Some(idx) => {
                self.entries[idx] = Entry { info, source };
            }
            None => {
                self.entries.push(Entry { info, source });
            }
        }

        Ok(())
    }

    /// Register all `.toml` files in a directory as custom themes.
    pub fn add_dir(&mut self, dir: &Path) -> Result<(), PaletteError> {
        let dir_str: Arc<str> = Arc::from(dir.to_string_lossy().as_ref());
        let read_dir = std::fs::read_dir(dir).map_err(|source| PaletteError::Io {
            path: Arc::clone(&dir_str),
            source,
        })?;

        for entry in read_dir {
            let entry = entry.map_err(|source| PaletteError::Io {
                path: Arc::clone(&dir_str),
                source,
            })?;
            let path = entry.path();
            match path.extension().and_then(|e| e.to_str()) {
                Some("toml") => self.add_file(&path)?,
                _ => continue,
            }
        }

        Ok(())
    }
}

impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}

impl Registry {
    fn find_entry(&self, id: &str) -> Result<&Entry, PaletteError> {
        self.entries
            .iter()
            .rfind(|e| e.info.id.as_ref() == id)
            .ok_or_else(|| PaletteError::UnknownPreset(Arc::from(id)))
    }

    fn toml_for(&self, id: &str) -> Result<&str, PaletteError> {
        let entry = self.find_entry(id)?;
        match &entry.source {
            Source::Builtin => {
                preset_toml(id).ok_or_else(|| PaletteError::UnknownPreset(Arc::from(id)))
            }
            Source::Custom(toml) => Ok(toml),
        }
    }

    fn resolve_manifest(&self, id: &str) -> Result<PaletteManifest, PaletteError> {
        PaletteManifest::from_toml(self.toml_for(id)?)
    }
}

fn extract_theme_info(toml_str: &str) -> Result<ThemeInfo, PaletteError> {
    let manifest = PaletteManifest::from_toml(toml_str)?;
    let meta = manifest.meta.ok_or(PaletteError::MissingMeta)?;
    Ok(ThemeInfo {
        id: meta.preset_id,
        name: meta.name,
        style: meta.style,
    })
}
