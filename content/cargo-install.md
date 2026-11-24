# cargo install

Build and install a Rust binary.

## Usage
- `cargo install package@version` : install a package from <https://crates.io> (the version is optional - latest by default)
- `cargo install --git repo_url` : install a package from the specified Git repository
- `cargo install --git repo_url --branch|tag|rev branch_name|tag|commit_hash` : build from the specified branch/tag/commit when installing from a Git repository
- `cargo install --path path/to/package` : install a package from a local directory
- `cargo install --list` : list all installed packages and their versions

## Categories
packages, version-control, development

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-install.html
- tldr page: https://tldr.inbrowser.app/pages/common/cargo-install
- Wikipedia: https://en.wikipedia.org/wiki/Rust_(programming_language)
