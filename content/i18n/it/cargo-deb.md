# cargo deb

Crea pacchetti Debian da progetti Cargo. Nota: questo non è un comando integrato di Cargo; va installato separatamente.

## Usage
- `cargo deb` : crea un pacchetto Debian da un progetto
- `cargo deb -o percorso/del/file_o_directory` : scrivi il file `.deb` nella directory o file specificato
- `cargo deb --target x86_64-unknown-linux-gnu` : compila per il target Rust specificato
- `cargo deb -p package_name` : seleziona quale pacchetto usare in un workspace Cargo
- `cargo deb --install` : installa immediatamente il pacchetto creato

## Links
- More information: https://github.com/kornelski/cargo-deb
