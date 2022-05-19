# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

<!-- next-header -->
## [Unreleased] - ReleaseDate

## [0.2.1] - 2022-05-19

### Features

- With `From` and `PartialEq`, colors and `Effects` are considered equivalent of `Style`

## [0.2.0] - 2022-05-19

### Breaking Changes

- `Style::fg_color` / `Style::bg_color` now accept an `Option<Color>`

### Features

- `no_std` support
- `let style = color | effects;` support for easier construction
- Added `Color::from((r, g, b))` for easier creation

### Fixes

- Allow clearing fg/bg colors

## [0.1.1] - 2022-05-18

<!-- next-url -->
[Unreleased]: https://github.com/rust-cli/anstyle/compare/v0.2.1...HEAD
[0.2.1]: https://github.com/rust-cli/anstyle/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/rust-cli/anstyle/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/rust-cli/anstyle/compare/6644c8911424a1451b483d39a3b415a41abfdf1b...v0.1.1
