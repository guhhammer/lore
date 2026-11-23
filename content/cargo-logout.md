# cargo logout

Remove an API token from the registry locally. The token is used to authenticate to a package registry. You can add it back using `cargo login`.

## Usage
- `cargo logout` : remove an API token from the local credential storage (located in `$CARGO_HOME/credentials.toml`)
- `cargo logout --registry name` : use the specified registry (registry names can be defined in the configuration - the default is <https://crates.io>)

## Categories
network, packages, users

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-logout.html
- tldr page: https://tldr.inbrowser.app/pages/common/cargo-logout
