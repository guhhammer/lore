# cargo check

Check a local package and all of its dependencies for errors.

## Usage
- `cargo c` : check the current package
- `cargo c --tests` : check all tests
- `cargo c --test integration_test1` : check the integration tests in `tests/integration_test1.rs`
- `cargo c -F feature1,feature2` : check the current package with the features `feature1` and `feature2`
- `cargo c --no-default-features` : check the current package with default features disabled

## Categories
packages, development

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-check.html
- tldr page: https://tldr.inbrowser.app/pages/common/cargo-check
- Wikipedia: https://en.wikipedia.org/wiki/Rust_(programming_language)
