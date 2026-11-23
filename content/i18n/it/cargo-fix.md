# cargo fix

Correggi automaticamente i warning dei lint segnalati da `rustc`.

## Usage
- `cargo fix --broken-code` : correggi il codice anche se contiene già errori del compilatore
- `cargo fix --allow-dirty` : correggi il codice anche se la directory di lavoro ha modifiche non committate
- `cargo fix --edition` : migra un pacchetto alla prossima edition di Rust
- `cargo fix --lib` : correggi la libreria del pacchetto
- `cargo fix --test nome` : correggi il test di integrazione specificato
- `cargo fix --workspace` : correggi tutti i membri dello workspace

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-fix.html
