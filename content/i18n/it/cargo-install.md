# cargo install

Compila e installa un binario Rust.

## Usage
- `cargo install pacchetto@versione` : installa un pacchetto da <https://crates.io> (la versione è opzionale - di default l'ultima)
- `cargo install --git repo_url` : installa un pacchetto dal repository Git specificato
- `cargo install --git repo_url --branch|tag|rev branch_name|tag|commit_hash` : compila dal branch/tag/commit specificato quando si installa da un repository Git
- `cargo install --path percorso/del/package` : installa un pacchetto da una directory locale
- `cargo install --list` : elenca tutti i pacchetti installati e le loro versioni

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-install.html
