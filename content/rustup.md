# rustup

Install, manage, and update Rust toolchains. Some subcommands such as `toolchain`, `target`, `update`, etc. have their own usage documentation.

## Usage
- `rustup install nightly` : install the nightly toolchain for your system
- `rustup default nightly` : switch the default toolchain to nightly so that the `cargo` and `rustc` commands will use it
- `rustup override set nightly` : use the nightly toolchain when inside the current project but leave global settings unchanged
- `rustup update` : update all toolchains
- `rustup show` : list installed toolchains
- `rustup run toolchain cargo build` : run `cargo build` with a certain toolchain
- `rustup doc` : open the local Rust documentation in the default web browser

## Categories
packages, documents, development

## Links
- More information: https://rust-lang.github.io/rustup/
- tldr page: https://tldr.inbrowser.app/pages/common/rustup
