# rustup target

Modify a toolchain's supported targets. Without the `--toolchain` option `rustup` will use the default toolchain. See `rustup help toolchain` for more information about toolchains.

## Usage
- `rustup target add --toolchain toolchain target` : add a target to a toolchain
- `rustup target remove --toolchain toolchain target` : remove a target from a toolchain
- `rustup target list --toolchain toolchain` : list available and installed targets for a toolchain
- `rustup target list --toolchain toolchain --installed` : list installed targets for a toolchain

## Links
- More information: https://rust-lang.github.io/rustup/
- tldr page: https://tldr.inbrowser.app/pages/common/rustup-target
