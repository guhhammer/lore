# cargo deb

Create Debian packages from Cargo projects. Note: This is not a built-in Cargo command, you need to install it first.

## Usage
- `cargo deb` : create a Debian package from a project
- `cargo deb -o path/to/file_or_directory` : write the `.deb` file to the specified file or directory
- `cargo deb --target x86_64-unknown-linux-gnu` : compile for the specified Rust target triple
- `cargo deb -p package_name` : select which package to use in a Cargo workspace
- `cargo deb --install` : immediately install the created package

## Categories
packages, files

## Links
- More information: https://github.com/kornelski/cargo-deb
- tldr page: https://tldr.inbrowser.app/pages/common/cargo-deb
- Wikipedia: https://en.wikipedia.org/wiki/Rust_(programming_language)
