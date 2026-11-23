# cargo binstall

Installa binari Rust da artifact CI. In mancanza di binari disponibili, esegue il fallback su `cargo install` (da sorgente). Nota: questo non è un comando integrato di Cargo; va installato separatamente.

## Usage
- `cargo binstall pacchetto` : installa un pacchetto da <https://crates.io>
- `cargo binstall pacchetto@versione` : installa una versione specifica di un pacchetto (di default l'ultima)
- `cargo binstall -y pacchetto` : installa un pacchetto disabilitando i prompt di conferma

## Links
- More information: https://github.com/cargo-bins/cargo-binstall
