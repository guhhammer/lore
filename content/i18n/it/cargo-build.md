# cargo build

Compila un pacchetto locale e tutte le sue dipendenze.

## Usage
- `cargo b` : compila il pacchetto o i pacchetti definiti dal manifesto `Cargo.toml` nella directory locale
- `cargo b -r` : compila gli artefatti in modalità release, con ottimizzazioni
- `cargo b --locked` : richiedi che `Cargo.lock` sia aggiornato
- `cargo b --workspace` : compila tutti i pacchetti nello workspace
- `cargo b -p pacchetto` : compila un pacchetto specifico
- `cargo b --bin nome` : compila solo il binario specificato
- `cargo b --test nome_test` : compila solo il test target specificato

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-build.html
