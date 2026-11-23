# cargo yank

Remove a pushed crate from the index. This should only be used when you accidentally release a significantly broken crate. Note: This does not remove any data. The crate is still present after a yank - this just prevents new projects from using it.

## Usage
- `cargo yank crate@version` : yank the specified version of a crate
- `cargo yank --undo crate@version` : undo a yank (i.e. allow downloading it again)
- `cargo yank --registry name crate@version` : use the specified registry (registry names can be defined in the configuration - the default is <https://crates.io>)

## Categories
network

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-yank.html
- tldr page: https://tldr.inbrowser.app/pages/common/cargo-yank
