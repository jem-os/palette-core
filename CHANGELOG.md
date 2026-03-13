# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.8.3](https://github.com/jostled-org/palette-core/compare/v0.8.2...v0.8.3) - 2026-03-13

### Added

- *(contrast)* add resolve_with_contrast to auto-fix WCAG violations

## [0.8.2](https://github.com/jostled-org/palette-core/compare/v0.8.1...v0.8.2) - 2026-03-12

### Other

- skip expensive jobs on doc-only changes

## [0.8.1](https://github.com/jostled-org/palette-core/compare/v0.8.0...v0.8.1) - 2026-03-12

### Other

- add style modifiers and sub-token CSS variable documentation

## [0.8.0](https://github.com/jostled-org/palette-core/compare/v0.7.3...v0.8.0) - 2026-03-12

### Added

- add syntax style modifiers and deduplicate field definitions

### Other

- simplify license wording in README

## [0.7.3](https://github.com/jostled-org/palette-core/compare/v0.7.2...v0.7.3) - 2026-03-11

### Other

- add downloads and deps.rs badges to README

## [0.7.2](https://github.com/jostled-org/palette-core/compare/v0.7.1...v0.7.2) - 2026-03-11

### Added

- *(surface)* populate focus color across all 31 presets

## [0.7.1](https://github.com/jostled-org/palette-core/compare/v0.7.0...v0.7.1) - 2026-03-08

### Added

- *(terminal)* add chromatic ANSI cycling and style builder

## [0.7.0](https://github.com/jostled-org/palette-core/compare/v0.6.10...v0.7.0) - 2026-03-08

### Other

- update variable count, fix css-variables API example, add resolved palette guide
- [**breaking**] address audit findings across ownership, perf, and conformance

## [0.6.10](https://github.com/jostled-org/palette-core/compare/v0.6.9...v0.6.10) - 2026-03-07

### Other

- resorted badges in README

## [0.6.9](https://github.com/jostled-org/palette-core/compare/v0.6.8...v0.6.9) - 2026-03-06

### Other

- *(readme)* add CI badge, update preset count, highlight Golden Hour

## [0.6.8](https://github.com/jostled-org/palette-core/compare/v0.6.7...v0.6.8) - 2026-03-06

### Other

- add CI workflow, cargo-deny, migrate paste to pastey, add missing rustdoc

## [0.6.7](https://github.com/jostled-org/palette-core/compare/v0.6.6...v0.6.7) - 2026-03-04

### Added

- add golden hour theme family and rewrite README

## [0.6.6](https://github.com/jostled-org/palette-core/compare/v0.6.5...v0.6.6) - 2026-03-01

### Other

- apply rustfmt to src and tests

## [0.6.5](https://github.com/jostled-org/palette-core/compare/v0.6.4...v0.6.5) - 2026-03-01

### Other

- add rustdoc comments across all public API surface

## [0.6.4](https://github.com/jostled-org/palette-core/compare/v0.6.3...v0.6.4) - 2026-03-01

### Fixed

- *(docs)* exclude wasm feature from docs.rs builds

### Other

- *(ci)* restore original release-plz workflow

## [0.6.3](https://github.com/jostled-org/palette-core/compare/v0.6.2...v0.6.3) - 2026-03-01

### Fixed

- *(ci)* quote expression in release job condition

### Other

- split release workflow into separate files
- run release job only when release PR merges

## [0.6.2](https://github.com/jostled-org/palette-core/compare/v0.6.1...v0.6.2) - 2026-02-28

### Other

- update repository URL to jostled-org

## [0.6.1](https://github.com/jem-os/palette-core/compare/v0.6.0...v0.6.1) - 2026-02-27

### Other

- update repository URL to jem-os org

## [0.6.0](https://github.com/jkoshiel/palette-core/compare/v0.5.1...v0.6.0) - 2026-02-25

### Added

- *(palette)* implement Default with neutral dark fallback
- *(css)* [**breaking**] split to_css into to_css, to_css_scoped, and to_css_custom_properties

## [0.5.1](https://github.com/jkoshiel/palette-core/compare/v0.5.0...v0.5.1) - 2026-02-25

### Added

- *(registry)* add infallible preset() for built-in themes

## [0.5.0](https://github.com/jkoshiel/palette-core/compare/v0.4.0...v0.5.0) - 2026-02-24

### Other

- [**breaking**] redesign CSS variable names, optional prefix, and iterator APIs

## [0.4.0](https://github.com/jkoshiel/palette-core/compare/v0.3.6...v0.4.0) - 2026-02-24

### Added

- [**breaking**] add wasm bindings, convenience API, and ownership cleanup

### Other

- updated README

## [0.3.6](https://github.com/jkoshiel/palette-core/compare/v0.3.5...v0.3.6) - 2026-02-23

### Other

- updated README tag

## [0.3.5](https://github.com/jkoshiel/palette-core/compare/v0.3.4...v0.3.5) - 2026-02-23

### Other

- updated README for multi-theme definition
- updated README to define CSS usage

## [0.3.4](https://github.com/jkoshiel/palette-core/compare/v0.3.3...v0.3.4) - 2026-02-23

### Added

- color manipulation

### Other

- updated README

## [0.3.3](https://github.com/jkoshiel/palette-core/compare/v0.3.2...v0.3.3) - 2026-02-23

### Added

- WCAG contrast ratio. Closes #7

## [0.3.2](https://github.com/jkoshiel/palette-core/compare/v0.3.1...v0.3.2) - 2026-02-23

### Other

- added NOTICE file with upstream attribution

## [0.3.1](https://github.com/jkoshiel/palette-core/compare/v0.3.0...v0.3.1) - 2026-02-23

### Added

- Runtime preset loading. Closes #6

## [0.3.0](https://github.com/jkoshiel/palette-core/compare/v0.2.1...v0.3.0) - 2026-02-23

### Added

- implement snapshot, platform features and expand egui mappings

## [0.2.1](https://github.com/jkoshiel/palette-core/compare/v0.2.0...v0.2.1) - 2026-02-23

### Other

- update bundled presets list in README

## [0.2.0] - 2026-02-22

### Added

- 18 new theme presets: Ayu (dark/light/mirage), Dracula, Everforest (dark/light), Gruvbox (dark/light), Kanagawa, Monokai, Nord, One (dark/light), Rose Pine (base/dawn/moon), Solarized (dark/light)
- `Display` and `Error` impls for `InvalidHex`

### Fixed

- Panic on non-ASCII hex input in `Color::from_hex`

### Changed

- Preset registry unified via macro (single source of truth)
- egui mapping uses `apply_color!` macro, reducing repetition
- CSS generation writes RGB directly to buffer (no intermediate allocation)
- Shared test helpers extracted to `tests/common.rs`

## [0.1.0] - 2026-02-22

Initial release.

### Added

- TOML-defined palette manifest with 8 sections (base, semantic, diff, surface, typography, syntax, editor, terminal)
- Palette inheritance and manifest merging
- 10 built-in presets (Catppuccin variants, GitHub, Tokyo Night variants)
- CSS custom property export
- egui `Visuals` mapping (feature: `egui`)
- ratatui terminal theme mapping (feature: `terminal`)
- Hex color parsing with `InvalidHex` error type
