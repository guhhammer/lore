# cargo run

Run the current Cargo package. Note: The working directory of the executed binary will be set to the current working directory.

## Usage
- `cargo r` : run the default binary target
- `cargo r --bin name` : run the specified binary
- `cargo r --example name` : run the specified example
- `cargo r -F "feature1 feature2 ..."` : activate a space or comma separated list of features
- `cargo r --no-default-features` : disable the default features
- `cargo r --all-features` : activate all available features
- `cargo r --profile name` : run with the given profile

## Categories
files, packages

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-run.html
- tldr page: https://tldr.inbrowser.app/pages/common/cargo-run
- Wikipedia: https://en.wikipedia.org/wiki/Rust_(programming_language)
