# cargo metadata

Output the workspace members and resolved dependencies of current package as JSON. Note: The output format is subject to change in future versions of Cargo.

## Usage
- `cargo metadata` : print the workspace members and resolved dependencies of the current package
- `cargo metadata --no-deps` : print only the workspace members and do not fetch dependencies
- `cargo metadata --format-version version` : print metadata in a specific format based on the specified version
- `cargo metadata --filter-platform target_triple` : print metadata with the `resolve` field including dependencies only for the given target triple (Note: The `packages` array will still include the dependencies for all targets)

## Categories
packages, documents, text

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-metadata.html
- tldr page: https://tldr.inbrowser.app/pages/common/cargo-metadata
