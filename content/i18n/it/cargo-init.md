# cargo init

Crea un nuovo pacchetto Cargo. Equivalente a `cargo new`, ma specificare una directory è opzionale.

## Usage
- `cargo init` : inizializza un progetto Rust con target binario nella directory corrente
- `cargo init percorso/della/directory` : inizializza un progetto Rust con target binario nella directory specificata
- `cargo init --lib` : inizializza un progetto Rust con target libreria nella directory corrente
- `cargo init --vcs git|hg|pijul|fossil|none` : inizializza un repository di controllo versione nella directory del progetto (predefinito: `git`)
- `cargo init --name nome` : imposta il nome del pacchetto (predefinito: nome della directory)

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-init.html
