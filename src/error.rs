use std::sync::Arc;

/// Errors produced when loading or parsing theme manifests.
#[derive(Debug, thiserror::Error)]
pub enum PaletteError {
    /// TOML deserialization failed.
    #[error("failed to parse manifest: {0}")]
    Parse(#[from] toml::de::Error),

    /// File read failed.
    #[error("failed to read {path}: {source}")]
    Io {
        /// Path that could not be read.
        path: Arc<str>,
        /// Underlying I/O error.
        source: std::io::Error,
    },

    /// Manifest has no `[base]` section.
    #[error("manifest missing required [base] section")]
    MissingBase,

    /// Manifest has no `[meta]` section (required for registry).
    #[error("manifest missing required [meta] section")]
    MissingMeta,

    /// A hex color string in the manifest is malformed.
    #[error("invalid hex `{value}` in [{section}].{field}")]
    InvalidHex {
        /// TOML section containing the bad value.
        section: Arc<str>,
        /// Field name within the section.
        field: Arc<str>,
        /// The malformed hex string.
        value: Arc<str>,
    },

    /// No built-in or registered preset matches the given ID.
    #[error("unknown preset: {0}")]
    UnknownPreset(Arc<str>),
}
