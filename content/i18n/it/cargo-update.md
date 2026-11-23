# cargo update

Aggiorna le dipendenze come registrate in `Cargo.lock`.

## Usage
- `cargo update` : aggiorna le dipendenze in `Cargo.lock` all'ultima versione possibile
- `cargo update -n` : mostra cosa verrebbe aggiornato, senza scrivere effettivamente il lockfile
- `cargo update --package dipendenza1 --package dipendenza2 --package dipendenza3` : aggiorna solo le dipendenze specificate
- `cargo update --package dipendenza --precise 1.2.3` : imposta una dipendenza specifica a una versione precisa

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-update.html
