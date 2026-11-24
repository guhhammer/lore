# cargo rustdoc

Build the documentation of Rust packages. Similar to `cargo doc`, but you can pass options to `rustdoc`. See `rustdoc --help` for all available options.

## Usage
- `cargo rustdoc -- rustdoc_options` : pass options to `rustdoc`
- `cargo rustdoc -- -W rustdoc::lint_name` : warn about a documentation lint
- `cargo rustdoc -- -A rustdoc::lint_name` : ignore a documentation lint
- `cargo rustdoc --lib` : document the package's library
- `cargo rustdoc --bin name` : document the specified binary
- `cargo rustdoc --example name` : document the specified example
- `cargo rustdoc --test name` : document the specified integration test

## See also
- rustdoc

## Categories
documents, development, packages

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-rustdoc.html
- tldr page: https://tldr.inbrowser.app/pages/common/cargo-rustdoc
- Wikipedia: https://en.wikipedia.org/wiki/Rust_(programming_language)
