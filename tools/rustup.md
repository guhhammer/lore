# rustup

rustup installs and updates the Rust toolchain (compiler, cargo, tools) and lets you switch between stable, beta and nightly or between targets.

## Tasks
### Install and update
- `rustup update` : update every installed toolchain
- `rustup update stable` : update only stable
- `rustup self update` : update rustup itself
- `rustup show` : show the active toolchain and targets
- `rustup --version` : check it is installed
### Toolchains
- `rustup toolchain list` : show installed toolchains
- `rustup toolchain install nightly` : add the nightly toolchain
- `rustup toolchain install 1.88.0` : add an exact version
- `rustup toolchain uninstall nightly` : remove one (deletes it)
- `rustup default stable` : the toolchain used everywhere by default
- `rustup override set nightly` : use another toolchain in this folder only
- `rustup override unset` : go back to the default here
- `cargo +nightly build` : use one toolchain for one command
### Components
- `rustup component list` : components you can add
- `rustup component add clippy rustfmt` : add the linter and formatter
- `rustup component add rust-src` : source for editors and rust-analyzer
- `rustup component add rust-analyzer` : the language server
- `rustup component remove clippy` : take one out
### Targets
- `rustup target list --installed` : targets you can build for
- `rustup target add x86_64-unknown-linux-musl` : add a static Linux target
- `rustup target add aarch64-apple-darwin` : add an Apple silicon target
- `rustup target add wasm32-unknown-unknown` : add WebAssembly
### Per project and docs
- `rustup run stable rustc --version` : run a command with a toolchain
- `rustup doc --book` : open the Rust book offline
- `rustup which cargo` : the path of the cargo in use

## Notes
- A `rust-toolchain.toml` file in a project pins the toolchain and components for everyone.
- Programs live in `~/.cargo/bin`; make sure that folder is in PATH.
- Install rustup from the official page in Links, not from a distro copy, if you want the normal workflow.
- Cross-compiling also needs a linker for the target.

## See also
- cargo
- rustc

## Links
- Install: https://www.rust-lang.org/tools/install
- Book: https://rust-lang.github.io/rustup/

## Categories
packages development
