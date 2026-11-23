# cargo clean

Rimuovi gli artefatti generati nella directory `target`.

## Usage
- `cargo clean` : rimuovi l'intera directory `target`
- `cargo clean --doc` : rimuovi gli artefatti della documentazione (la directory `target/doc`)
- `cargo clean -r` : rimuovi gli artefatti di release (la directory `target/release`)
- `cargo clean --profile dev` : rimuovi gli artefatti nella directory del profilo specificato (in questo caso `target/debug`)

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-clean.html
