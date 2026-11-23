# cargo login

Save an API token from the registry locally. The token is used to authenticate to a package registry. You can remove it using `cargo logout`.

## Usage
- `cargo login` : add an API token to the local credential storage (located in `$CARGO_HOME/credentials.toml`)
- `cargo login --registry name` : use the specified registry (registry names can be defined in the configuration - the default is <https://crates.io>)

## Categories
users, network, packages

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-login.html
- tldr page: https://tldr.inbrowser.app/pages/common/cargo-login
