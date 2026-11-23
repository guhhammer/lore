# cargo publish

Carica un pacchetto su un registro. Nota: devi aggiungere un token di autenticazione con `cargo login` prima di pubblicare un pacchetto.

## Usage
- `cargo publish` : esegui i controlli, crea un file `.crate` e caricalo sul registro
- `cargo publish -n` : esegui i controlli, crea un file `.crate` ma non caricarlo (equivalente a `cargo package`)
- `cargo publish --registry nome` : usa il registro specificato (i nomi dei registri possono essere definiti nella configurazione - il predefinito è <https://crates.io>)

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-publish.html
