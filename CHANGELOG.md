# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased
### Removed
- `crate_plugin` deleted in favor of passing `--lib` to `cargo-generate`.

### Added
- Template now reacts to being expanded inside a `cargo` project. If `cargo-generate` detects a 
`Cargo.toml` file in a previous folder, the template will not include several unneeded files.

### Changed
- Always include a `setup` startup system when expanding for a binary.
- Inclusion of additional files like `CHANGELOG.md' is now optional.
- Requires `cargo-generate` 0.15.0

## [v1.3.0]
### Added
- VSCode tasks now utilizes Bevy *fast compiles* on Linux.
- Updated integration test example with better example, using a proper `App`, with disabled
backend so it also works on GitHub actions.

### Changed
- Renamed `lib-plugin` to `crate-plugin`. Why? Because in rust we use crates, not libraries!

### Removed
- Deleted the sub-template `lib-assets`.

[v1.3.0]: https://github.com/