# cargo init

Create a new Cargo package. Equivalent of `cargo new`, but specifying a directory is optional.

## Usage
- `cargo init` : initialize a Rust project with a binary target in the current directory
- `cargo init path/to/directory` : initialize a Rust project with a binary target in the specified directory
- `cargo init --lib` : initialize a Rust project with a library target in the current directory
- `cargo init --vcs git|hg|pijul|fossil|none` : initialize a version control system repository in the project directory (default: `git`)
- `cargo init --name name` : set the package name (default: directory name)

## Categories
files, development, packages

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-init.html
- tldr page: https://tldr.inbrowser.app/pages/common/cargo-init
- Wikipedia: https://en.wikipedia.org/wiki/Rust_(programming_language)
