# rustup which

Display which binary will be run for a command managed by `rustup`. Like `which`, but searches a Rust toolchain instead of `$PATH`.

## Usage
- `rustup which command` : display the path to the binary in the default toolchain
- `rustup which --toolchain toolchain command` : display the path to the binary in the specified toolchain (see `rustup help toolchain` for more information)

## Categories
desktop, files

## Links
- More information: https://rust-lang.github.io/rustup/
- tldr page: https://tldr.inbrowser.app/pages/common/rustup-which
