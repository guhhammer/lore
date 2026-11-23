# cargo

Gestisce progetti Rust e le loro dipendenze (crate). Alcuni sottocomandi come `build` hanno la propria documentazione.

## Usage
- `cargo search stringa_di_ricerca` : cerca crate
- `cargo install nome_crate` : installa un crate binario
- `cargo install --list` : elenca i crate binari installati
- `cargo init --bin|lib percorso/della/directory` : crea un nuovo progetto Rust binario o libreria nella directory specificata
- `cargo add dipendenza` : aggiungi una dipendenza a `Cargo.toml` nella directory corrente
- `cargo b -r` : compila il progetto Rust con il profilo release
- `cargo +nightly b` : compila il progetto con il compilatore nightly (richiede `rustup`)
- `cargo b -j numero_thread` : compila usando un numero specifico di thread

## Links
- More information: https://doc.rust-lang.org/stable/cargo/
