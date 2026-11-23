# rustup override

Modify directory toolchain overrides. See `rustup help toolchain` for more information about toolchains.

## Usage
- `rustup override list` : list directory toolchain overrides
- `rustup override set toolchain` : set the override toolchain for the current directory (i.e. tell `rustup` to run `cargo`, `rustc`, etc. from a specific toolchain when in that directory)
- `rustup override unset` : remove the toolchain override for the current directory
- `rustup override unset --nonexistent` : remove all toolchain overrides for directories that no longer exist

## Categories
files

## Links
- More information: https://rust-lang.github.io/rustup/
- tldr page: https://tldr.inbrowser.app/pages/common/rustup-override
