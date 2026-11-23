# cargo msrv

Gestisci la Minimum Supported Rust Version (MSRV) di un progetto. Nota: questo non è un comando integrato di Cargo; va installato separatamente.

## Usage
- `cargo msrv list` : mostra le MSRV delle dipendenze (come specificato nei loro `Cargo.toml`)
- `cargo msrv find` : trova la MSRV del progetto provando a compilarlo con toolchain differenti
- `cargo msrv show` : mostra la MSRV del progetto come specificata in `Cargo.toml`
- `cargo msrv set versione` : imposta la MSRV in `Cargo.toml` a una versione di Rust data
- `cargo msrv verify` : verifica se la MSRV è soddisfacibile compilando il progetto con la versione specificata di Rust

## Links
- More information: https://gribnau.dev/cargo-msrv/
