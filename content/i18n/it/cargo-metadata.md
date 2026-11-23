# cargo metadata

Stampa i membri del workspace e le dipendenze risolte del pacchetto corrente in formato JSON. Nota: il formato di output potrebbe cambiare in versioni future di Cargo.

## Usage
- `cargo metadata` : stampa i membri del workspace e le dipendenze risolte del pacchetto corrente
- `cargo metadata --no-deps` : stampa solo i membri del workspace senza recuperare le dipendenze
- `cargo metadata --format-version versione` : stampa i metadata in un formato specifico basato sulla versione specificata
- `cargo metadata --filter-platform target_triple` : stampa i metadata con il campo `resolve` includendo le dipendenze solo per il target dato (Nota: l'array `packages` continuerà ad includere le dipendenze per tutti i target)

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-metadata.html
