# cargo check

Controlla un pacchetto locale e tutte le sue dipendenze alla ricerca di errori.

## Usage
- `cargo c` : controlla il pacchetto corrente
- `cargo c --tests` : controlla tutti i test
- `cargo c --test integration_test1` : controlla l'integration test `tests/integration_test1.rs`
- `cargo c -F feature1,feature2` : controlla il pacchetto corrente con le feature `feature1` e `feature2`
- `cargo c --no-default-features` : controlla il pacchetto corrente con le feature di default disabilitate

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-check.html
