# cargo

cargo is Rust's build tool and package manager: it makes projects, fetches crates, builds, tests and publishes.

## Tasks
### Start
- `cargo new NAME` : make a new binary project
- `cargo new NAME --lib` : make a library project
- `cargo init` : turn the current folder into a project
### Build and run
- `cargo build` : compile in debug mode
- `cargo build --release` : compile with optimizations
- `cargo run` : build and run
- `cargo run --release -- ARGS` : run an optimized build with arguments
- `cargo check` : check that it compiles, faster than a build
- `cargo clean` : delete the target folder
### Test and tidy
- `cargo test` : run the tests
- `cargo test NAME` : run tests whose name contains NAME
- `cargo test -- --nocapture` : show the tests' printed output
- `cargo fmt` : format the code
- `cargo clippy` : run the lints
- `cargo clippy -- -D warnings` : make lint warnings fail
- `cargo doc --open` : build the docs and open them
- `cargo bench` : run the benchmarks
### Dependencies
- `cargo add serde` : add a crate to Cargo.toml
- `cargo add serde --features derive` : add it with a feature
- `cargo remove serde` : take a crate out
- `cargo update` : update Cargo.lock within allowed versions
- `cargo tree` : show the dependency tree
- `cargo search NAME` : look a crate up on crates.io
### Install tools
- `cargo install ripgrep` : build and install a program from crates.io
- `cargo install --path .` : install the current project
- `cargo install --list` : show installed programs
- `cargo uninstall NAME` : remove one
### Publish
- `cargo login` : save your crates.io token
- `cargo publish --dry-run` : check a release without uploading
- `cargo publish` : upload to crates.io

## Notes
- Programs end up in `~/.cargo/bin`: put it in your PATH.
- Commit `Cargo.lock` for programs; libraries often leave it out.
- `cargo check` is the quick loop while editing; save `build` for running.
- `cargo add`, `remove` and `tree` come with current cargo; older versions need cargo-edit.

## See also
- rustup
- rustc
- make

## Links
- The Cargo Book: https://doc.rust-lang.org/cargo/
- Crates: https://crates.io

## Categories
packages development
