# cargo build

Compile a local package and all of its dependencies.

## Usage
- `cargo b` : build the package or packages defined by the `Cargo.toml` manifest file in the local path
- `cargo b -r` : build artifacts in release mode, with optimizations
- `cargo b --locked` : require that `Cargo.lock` is up to date
- `cargo b --workspace` : build all packages in the workspace
- `cargo b -p package` : build a specific package
- `cargo b --bin name` : build only the specified binary
- `cargo b --test test_name` : build only the specified test target

## Categories
development, packages

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-build.html
- tldr page: https://tldr.inbrowser.app/pages/common/cargo-build
