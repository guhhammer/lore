# cargo package

Assemble a local package into a distributable tarball (a `.crate` file). Similar to `cargo publish --dry-run`, but has more options.

## Usage
- `cargo package` : perform checks and create a `.crate` file (equivalent of `cargo publish --dry-run`)
- `cargo package -l` : display what files would be included in the tarball without actually creating it

## Categories
packages, files

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-package.html
- tldr page: https://tldr.inbrowser.app/pages/common/cargo-package
