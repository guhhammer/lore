# rustup set

Alter `rustup` settings.

## Usage
- `rustup set default-host host_triple` : set the default host triple
- `rustup set profile minimal|default` : set the default profile (`minimal` includes only `rustc`, `rust-std`, and `cargo`, whereas `default` adds `rust-docs`, `rustfmt`, and `clippy`)
- `rustup set auto-self-update enable|disable|check-only` : set whether `rustup` should update itself when running `rustup update`

## Categories
network

## Links
- More information: https://rust-lang.github.io/rustup/
- tldr page: https://tldr.inbrowser.app/pages/common/rustup-set
