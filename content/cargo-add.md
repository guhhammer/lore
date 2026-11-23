# cargo add

Add dependencies to a Rust project's `Cargo.toml` manifest.

## Usage
- `cargo add dependency` : add the latest version of a dependency to the current project
- `cargo add dependency@version` : add a specific version of a dependency
- `cargo add dependency -F feature_1,feature_2,...` : add a dependency and enable one or more specific features
- `cargo add dependency --optional` : add an optional dependency, which then gets exposed as a feature of the crate
- `cargo add --path path/to/crate_directory` : add a local crate as a dependency
- `cargo add dependency --dev|build` : add a development or build dependency
- `cargo add dependency --no-default-features` : add a dependency with all default features disabled

## Categories
development, packages

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-add.html
- tldr page: https://tldr.inbrowser.app/pages/common/cargo-add
