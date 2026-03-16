# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/)
and this project adheres to [Semantic Versioning](https://semver.org/).

<!-- next-header -->
## [Unreleased] - ReleaseDate

## [0.1.0] - 2026-03-16

### Breaking changes

- Instead of using `alternate` to remove, use `TermProgressStatus::Removed`
- `TermProgress::percent` no longer takes an `Option`

### Features

- Added `TermProgress::start`
- Added `TermProgress::error`
- Added `TermProgress::remove`

### Fixes

- Only include percent when needed
- Removed iTerm check for `TERM_FEATURES`

## [0.0.1] - 2026-03-16

Initial release

<!-- next-url -->
[Unreleased]: https://github.com/rust-cli/anstyle/compare/anstyle-progress-v0.1.0...HEAD
[0.1.0]: https://github.com/rust-cli/anstyle/compare/anstyle-progress-v0.0.1...anstyle-progress-v0.1.0
[0.0.1]: https://github.com/rust-cli/anstyle/compare/8ed060819922c65d8bdf37bb83442db64579c53f...anstyle-progress-v0.0.1
