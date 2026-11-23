# rustup component

Modify a toolchain's installed components. Without the `--toolchain` option `rustup` will use the default toolchain. See `rustup help toolchain` for more information about toolchains.

## Usage
- `rustup component add --toolchain toolchain component` : add a component to a toolchain
- `rustup component remove --toolchain toolchain component` : remove a component from a toolchain
- `rustup component list --toolchain toolchain` : list installed and available components for a toolchain
- `rustup component list --toolchain toolchain --installed` : list installed components for a toolchain

## Links
- More information: https://rust-lang.github.io/rustup/
- tldr page: https://tldr.inbrowser.app/pages/common/rustup-component
