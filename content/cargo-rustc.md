# cargo rustc

Compile a Rust package. Similar to `cargo build`, but you can pass extra options to the compiler. See `rustc --help` for all available options.

## Usage
- `cargo rustc -- rustc_options` : build the package and pass options to `rustc`
- `cargo rustc -r` : build artifacts in release mode, with optimizations
- `cargo rustc -r -- -C target-cpu=native` : compile with architecture-specific optimizations for the current CPU
- `cargo rustc -- -C opt-level=1|2|3` : compile with speed optimizations
- `cargo rustc -- -C opt-level=s|z` : compile with size optimizations (`z` also turns off loop vectorization)
- `cargo rustc --lib -- -D unsafe-code` : check if your package uses unsafe code
- `cargo rustc -p package` : build a specific package
- `cargo rustc --bin name` : build only the specified binary

## See also
- rustc

## Categories
development, packages

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-rustc.html
- tldr page: https://tldr.inbrowser.app/pages/common/cargo-rustc
- Wikipedia: https://en.wikipedia.org/wiki/Rust_(programming_language)
