# cargo clippy

A collection of lints to catch common mistakes and improve your Rust code.

## Usage
- `cargo clippy` : run checks over the code in the current directory
- `cargo clippy --locked` : require that `Cargo.lock` is up to date
- `cargo clippy --workspace` : run checks on all packages in the workspace
- `cargo clippy --package package` : run checks for a package
- `cargo clippy -- -W clippy::lint_group` : run checks for a lint group (see <https://rust-lang.github.io/rust-clippy/stable/index.html#?groups=cargo,complexity,correctness,deprecated,nursery,pedantic,perf,restriction,style,suspicious>)
- `cargo clippy -- -D warnings` : treat warnings as errors
- `cargo clippy -- -A warnings` : run checks and ignore warnings
- `cargo clippy --fix` : apply Clippy suggestions automatically

## Categories
packages, network

## Links
- More information: https://github.com/rust-lang/rust-clippy
- tldr page: https://tldr.inbrowser.app/pages/common/cargo-clippy
- Wikipedia: https://en.wikipedia.org/wiki/Rust_(programming_language)
