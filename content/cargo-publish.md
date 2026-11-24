# cargo publish

Upload a package to a registry. Note: You have to add an authentication token using `cargo login` before publishing a package.

## Usage
- `cargo publish` : perform checks, create a `.crate` file, and upload it to the registry
- `cargo publish -n` : simulate publishing, performing checks, and creating a `.crate` file without uploading it (equivalent of `cargo package`)
- `cargo publish --registry name` : use the specified registry (registry names can be defined in the configuration - the default is <https://crates.io>)

## Categories
network, packages, users

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-publish.html
- tldr page: https://tldr.inbrowser.app/pages/common/cargo-publish
- Wikipedia: https://en.wikipedia.org/wiki/Rust_(programming_language)
