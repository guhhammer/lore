# cargo login

Salva un token API del registro localmente. Il token viene usato per autenticarsi verso un registro di pacchetti. Puoi rimuoverlo con `cargo logout`.

## Usage
- `cargo login` : aggiungi un token API allo storage delle credenziali locali (situato in `$CARGO_HOME/credentials.toml`)
- `cargo login --registry nome` : usa il registro specificato (i nomi dei registri possono essere definiti nella configurazione - il predefinito è <https://crates.io>)

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-login.html
