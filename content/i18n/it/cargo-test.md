# cargo test

Esegui i test unitari e di integrazione di un pacchetto Rust.

## Usage
- `cargo t nome_test` : esegui solo i test che contengono una stringa specifica nel loro nome
- `cargo t -- --test-threads conto` : imposta il numero di casi di test eseguiti contemporaneamente
- `cargo t -r` : esegui i test in modalità release, con ottimizzazioni
- `cargo t --workspace` : esegui i test per tutti i pacchetti nello workspace
- `cargo t -p pacchetto` : esegui i test per un pacchetto specifico
- `cargo t -- --nocapture` : esegui i test senza nascondere l'output delle esecuzioni dei test

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-test.html
