# cargo owner

Manage the owners of a crate on the registry.

## Usage
- `cargo owner -a username|github:org_name:team_name crate` : invite the given user or team as an owner
- `cargo owner -r username|github:org_name:team_name crate` : remove the given user or team as an owner
- `cargo owner -l crate` : list owners of a crate
- `cargo owner --registry name` : use the specified registry (registry names can be defined in the configuration - the default is <https://crates.io>)

## Categories
network

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-owner.html
- tldr page: https://tldr.inbrowser.app/pages/common/cargo-owner
- Wikipedia: https://en.wikipedia.org/wiki/Rust_(programming_language)
