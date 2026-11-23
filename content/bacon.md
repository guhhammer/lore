# bacon

A background code checker for Rust.

## Usage
- `bacon` : run `cargo check` whenever a change is detected in the current directory
- `bacon test path/to/directory` : run `cargo test` whenever a change is detected in the given directory
- `bacon check-all` : run `cargo check` against all targets whenever a change is detected in the current directory
- `bacon run|test|clippy|doc|...` : run a specific job whenever a change is detected in the current directory
- `bacon --list-jobs` : list all currently available jobs
- `bacon --init` : initialize a `bacon.toml` configuration file in the current directory

## Categories
files, development, processes

## Links
- More information: https://github.com/Canop/bacon
- tldr page: https://tldr.inbrowser.app/pages/common/bacon
