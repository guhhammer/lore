# cargo package

Assembla un pacchetto locale in un tarball distribuibile (un file `.crate`). Simile a `cargo publish --dry-run`, ma offre più opzioni.

## Usage
- `cargo package` : esegui i controlli e crea un file `.crate` (equivalente a `cargo publish --dry-run`)
- `cargo package -l` : mostra quali file sarebbero inclusi nel tarball senza crearli effettivamente

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-package.html
