# cargo rustdoc

Costruisci la documentazione dei pacchetti Rust. Simile a `cargo doc`, ma permette di passare opzioni a `rustdoc`. Vedi `rustdoc --help` per tutte le opzioni disponibili.

## Usage
- `cargo rustdoc -- rustdoc_options` : passa opzioni a `rustdoc`
- `cargo rustdoc -- -W rustdoc::lint_name` : segnala un lint della documentazione
- `cargo rustdoc -- -A rustdoc::lint_name` : ignora un lint della documentazione
- `cargo rustdoc --lib` : documenta la libreria del pacchetto
- `cargo rustdoc --bin nome` : documenta il binario specificato
- `cargo rustdoc --example nome` : documenta l'example specificato
- `cargo rustdoc --test nome` : documenta il test di integrazione specificato

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-rustdoc.html
