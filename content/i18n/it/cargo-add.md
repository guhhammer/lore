# cargo add

Aggiungi dipendenze al manifesto `Cargo.toml` di un progetto Rust.

## Usage
- `cargo add dipendenza` : aggiungi la versione più recente di una dipendenza al progetto corrente
- `cargo add dipendenza@versione` : aggiungi una versione specifica di una dipendenza
- `cargo add dipendenza -F feature_1,feature_2,...` : aggiungi una dipendenza e abilita una o più feature specifiche
- `cargo add dipendenza --optional` : aggiungi una dipendenza opzionale, che verrà esposta come feature del crate
- `cargo add --path path/della/crate_directory` : aggiungi un crate locale come dipendenza
- `cargo add dipendenza --dev|build` : aggiungi una dipendenza di sviluppo o di build
- `cargo add dipendenza --no-default-features` : aggiungi una dipendenza disabilitando tutte le feature di default

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-add.html
