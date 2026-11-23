# cargo clean

Remove generated artifacts in the `target` directory.

## Usage
- `cargo clean` : remove the entire `target` directory
- `cargo clean --doc` : remove documentation artifacts (the `target/doc` directory)
- `cargo clean -r` : remove release artifacts (the `target/release` directory)
- `cargo clean --profile dev` : remove artifacts in the directory of the given profile (in this case, `target/debug`)

## Categories
files

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-clean.html
- tldr page: https://tldr.inbrowser.app/pages/common/cargo-clean
