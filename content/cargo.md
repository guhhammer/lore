# cargo

Manage Rust projects and their module dependencies (crates). Some subcommands such as `build` have their own usage documentation.

## Usage
- `cargo search search_string` : search for crates
- `cargo install crate_name` : install a binary crate
- `cargo install --list` : list installed binary crates
- `cargo init --bin|lib path/to/directory` : create a new binary or library Rust project in the specified directory (or the current working directory by default)
- `cargo add dependency` : add a dependency to `Cargo.toml` in the current directory
- `cargo b -r` : build the Rust project in the current directory using the release profile
- `cargo r` : run the built binary (will build it if it's not already built)
- `cargo +nightly b` : build the Rust project in the current directory using the nightly compiler (requires `rustup`)

## Categories
packages, development

## Links
- More information: https://doc.rust-lang.org/stable/cargo/
- tldr page: https://tldr.inbrowser.app/pages/common/cargo
- Wikipedia: https://en.wikipedia.org/wiki/Rust_(programming_language)
