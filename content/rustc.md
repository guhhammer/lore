# rustc

The Rust compiler. Rust projects usually use `cargo` instead of invoking `rustc` directly.

## Usage
- `rustc path/to/main.rs` : compile a binary crate
- `rustc -C lto -C opt-level=0|1|2|3|s|z path/to/main.rs` : compile with optimizations (`s` means optimize for binary size; `z` is the same with even more optimizations)
- `rustc -g path/to/main.rs` : compile with debugging information
- `rustc --explain error_code` : explain an error message
- `rustc -C target-cpu=native path/to/main.rs` : compile with architecture-specific optimizations for the current CPU
- `rustc --print target-list` : display the target list (Note: You have to add a target using `rustup` first to be able to compile for it)
- `rustc --target target_triple path/to/main.rs` : compile for a specific target
- `rustc -h` : display help

## Categories
development, files

## Links
- More information: https://doc.rust-lang.org/stable/rustc/
- tldr page: https://tldr.inbrowser.app/pages/common/rustc
