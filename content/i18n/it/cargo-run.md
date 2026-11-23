# cargo run

Esegui il pacchetto Cargo corrente. Nota: la directory di lavoro dell'eseguibile sarà impostata sulla directory di lavoro corrente.

## Usage
- `cargo r` : esegui il target binario predefinito
- `cargo r --bin nome` : esegui il binario specificato
- `cargo r --example nome` : esegui l'example specificato
- `cargo r -F "feature1 feature2 ..."` : attiva una lista di feature separate da spazio o virgola
- `cargo r --no-default-features` : disabilita le feature di default
- `cargo r --all-features` : attiva tutte le feature disponibili
- `cargo r --profile nome` : esegui con il profilo specificato

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-run.html
