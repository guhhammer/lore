# cargo test

Execute the unit and integration tests of a Rust package.

## Usage
- `cargo t test_name` : only run tests containing a specific string in their names
- `cargo t -- --test-threads count` : set the number of simultaneous running test cases
- `cargo t -r` : test artifacts in release mode, with optimizations
- `cargo t --workspace` : test all packages in the workspace
- `cargo t -p package` : run tests for a specific package
- `cargo t -- --nocapture` : run tests without hiding output from test executions

## Categories
development, packages

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-test.html
- tldr page: https://tldr.inbrowser.app/pages/common/cargo-test
- Wikipedia: https://en.wikipedia.org/wiki/Rust_(programming_language)
