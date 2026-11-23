# cargo rustc

Compila un pacchetto Rust. Simile a `cargo build`, ma permette di passare opzioni aggiuntive al compilatore. Vedi `rustc --help` per tutte le opzioni disponibili.

## Usage
- `cargo rustc -- rustc_options` : compila il pacchetto e passa opzioni a `rustc`
- `cargo rustc -r` : compila gli artefatti in modalità release
- `cargo rustc -r -- -C target-cpu=native` : compila con ottimizzazioni specifiche per l'architettura della CPU corrente
- `cargo rustc -- -C opt-level=1|2|3` : compila con ottimizzazioni per la velocità
- `cargo rustc -- -C opt-level=s|z` : compila con ottimizzazioni per la dimensione (`s` disattiva anche la vettorizzazione dei loop)
- `cargo rustc --lib -- -D unsafe-code` : controlla se il pacchetto utilizza codice `unsafe`
- `cargo rustc -p pacchetto` : compila un pacchetto specifico
- `cargo rustc --bin nome` : compila solo il binario specificato

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-rustc.html
