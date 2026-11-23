# cargo clippy

Una raccolta di lint per catturare errori comuni e migliorare il codice Rust.

## Usage
- `cargo clippy` : esegui i controlli sul codice nella directory corrente
- `cargo clippy --locked` : richiedi che `Cargo.lock` sia aggiornato
- `cargo clippy --workspace` : esegui i controlli su tutti i pacchetti nello workspace
- `cargo clippy --package pacchetto` : esegui i controlli per un pacchetto
- `cargo clippy -- -W clippy::lint_group` : esegui i controlli per un gruppo di lint (vedi <https://rust-lang.github.io/rust-clippy/stable/index.html#?groups=cargo,complexity,correctness,deprecated,nursery,pedantic,perf,restriction,style,suspicious>)
- `cargo clippy -- -D warnings` : tratta i warning come errori
- `cargo clippy -- -A warnings` : esegui i controlli ignorando i warning
- `cargo clippy --fix` : applica automaticamente i suggerimenti di Clippy

## Links
- More information: https://github.com/rust-lang/rust-clippy
