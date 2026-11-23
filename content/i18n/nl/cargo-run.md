# cargo run

Voer het huidige Cargo-pakket uit. Opmerking: de werkmap van de uitgevoerde binary wordt ingesteld op de huidige werkmap.

## Usage
- `cargo r` : voer de standaard binary uit
- `cargo r --bin naam` : voer de opgegeven binary uit
- `cargo r --example naam` : voer het opgegeven voorbeeld uit
- `cargo r -F "feature1 feature2 ..."` : activeer een door spaties of komma's gescheiden lijst met features
- `cargo r --no-default-features` : schakel de standaardfeatures uit
- `cargo r --all-features` : activeer alle beschikbare features
- `cargo r --profile naam` : voer uit met het opgegeven profiel

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-run.html
