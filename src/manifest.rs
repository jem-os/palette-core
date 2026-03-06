use std::collections::BTreeMap;
use std::sync::Arc;

use serde::Deserialize;

use crate::error::PaletteError;

/// A single TOML section mapping slot names to hex color strings.
pub type ManifestSection = BTreeMap<Arc<str>, Arc<str>>;

/// Platform-keyed overrides, e.g. `[platform.macos]`.
pub type PlatformSections = BTreeMap<Arc<str>, ManifestSection>;

/// The `[meta]` section of a theme TOML file.
#[derive(Debug, Clone, Deserialize)]
pub struct ManifestMeta {
    /// Human-readable theme name.
    pub name: Arc<str>,
    /// Machine identifier used for lookups.
    pub preset_id: Arc<str>,
    /// Schema version string (e.g. `"1"`).
    pub schema_version: Arc<str>,
    /// Visual style tag: `"dark"`, `"light"`, etc.
    pub style: Arc<str>,
    /// Theme kind (e.g. `"base"`, `"variant"`).
    pub kind: Arc<str>,
    /// Parent preset ID for inheritance.
    #[serde(default)]
    pub inherits: Option<Arc<str>>,
    /// Upstream repository URL, if ported from another project.
    #[serde(default)]
    pub upstream_repo: Option<Arc<str>>,
}

/// Parsed but unresolved theme manifest.
///
/// Holds raw hex strings grouped by section. Convert to a [`Palette`](crate::Palette)
/// via [`Palette::from_manifest`](crate::Palette::from_manifest) after resolving
/// inheritance with [`merge_manifests`](crate::merge::merge_manifests).
#[derive(Debug, Clone)]
pub struct PaletteManifest {
    /// Theme identity and inheritance metadata.
    pub meta: Option<ManifestMeta>,
    /// Core background/foreground hex values.
    pub base: ManifestSection,
    /// Status color hex values (success, error, etc.).
    pub semantic: ManifestSection,
    /// Diff highlighting hex values.
    pub diff: ManifestSection,
    /// UI surface hex values (menus, sidebars, etc.).
    pub surface: ManifestSection,
    /// Text chrome hex values (comments, line numbers, etc.).
    pub typography: ManifestSection,
    /// Syntax token hex values.
    pub syntax: ManifestSection,
    /// Editor chrome hex values (cursor, selections, etc.).
    pub editor: ManifestSection,
    /// ANSI terminal color hex values.
    pub terminal: ManifestSection,
    /// Per-platform color overrides.
    #[cfg(feature = "platform")]
    pub platform: PlatformSections,
}

impl PaletteManifest {
    /// Parse a TOML string into a manifest. Requires a `[base]` section.
    pub fn from_toml(s: &str) -> Result<Self, PaletteError> {
        let raw: RawManifest = toml::from_str(s)?;

        match raw.base {
            None => Err(PaletteError::MissingBase),
            Some(base) => Ok(Self {
                meta: raw.meta,
                base,
                semantic: raw.semantic,
                diff: raw.diff,
                surface: raw.surface,
                typography: raw.typography,
                syntax: raw.syntax,
                editor: raw.editor,
                terminal: raw.terminal,
                #[cfg(feature = "platform")]
                platform: raw.platform,
            }),
        }
    }

    /// The parent preset ID if this manifest uses inheritance.
    pub fn inherits_from(&self) -> Option<&str> {
        self.meta.as_ref().and_then(|m| m.inherits.as_deref())
    }
}

#[derive(Deserialize)]
struct RawManifest {
    #[serde(default)]
    meta: Option<ManifestMeta>,
    #[serde(default)]
    base: Option<ManifestSection>,
    #[serde(default)]
    semantic: ManifestSection,
    #[serde(default)]
    diff: ManifestSection,
    #[serde(default)]
    surface: ManifestSection,
    #[serde(default)]
    typography: ManifestSection,
    #[serde(default)]
    syntax: ManifestSection,
    #[serde(default)]
    editor: ManifestSection,
    #[serde(default)]
    terminal: ManifestSection,
    #[cfg(feature = "platform")]
    #[serde(default)]
    platform: PlatformSections,
}
