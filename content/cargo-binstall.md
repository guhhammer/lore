# cargo binstall

Install Rust binaries from CI artifacts. Falls back to `cargo install` (from source code) if there are no binaries available. Note: This is not a built-in Cargo command, you need to install it first.

## Usage
- `cargo binstall package` : install a package from <https://crates.io>
- `cargo binstall package@version` : install a specific version of a package (latest by default)
- `cargo binstall -y package` : install a package and disable confirmation prompts

## Categories
packages, development, network

## Links
- More information: https://github.com/cargo-bins/cargo-binstall
- tldr page: https://tldr.inbrowser.app/pages/common/cargo-binstall
- Wikipedia: https://en.wikipedia.org/wiki/Rust_(programming_language)
