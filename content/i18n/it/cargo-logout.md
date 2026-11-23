# cargo logout

Rimuovi un token API del registro localmente. Il token viene usato per autenticarsi verso un registro di pacchetti. Puoi aggiungerlo nuovamente con `cargo login`.

## Usage
- `cargo logout` : rimuovi un token API dallo storage delle credenziali locali (situato in `$CARGO_HOME/credentials.toml`)
- `cargo logout --registry nome` : usa il registro specificato (i nomi dei registri possono essere definiti nella configurazione - il predefinito è <https://crates.io>)

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-logout.html
