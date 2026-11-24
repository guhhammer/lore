# cargo msrv

Manage the Minimum Supported Rust Version (MSRV) of a project. Note: This is not a built-in Cargo command, you need to install it first.

## Usage
- `cargo msrv list` : display the MSRVs of dependencies (as specified in their `Cargo.toml`)
- `cargo msrv find` : find the MSRV of the project by trying to compile it with various toolchains
- `cargo msrv show` : show the MSRV of the project as specified in `Cargo.toml`
- `cargo msrv set version` : set the MSRV in `Cargo.toml` to a given Rust version
- `cargo msrv verify` : verify whether the MSRV is satisfiable by compiling the project using the specified version of Rust

## Categories
development, packages

## Links
- More information: https://gribnau.dev/cargo-msrv/
- tldr page: https://tldr.inbrowser.app/pages/common/cargo-msrv
- Wikipedia: https://en.wikipedia.org/wiki/Rust_(programming_language)
