# cargo bench

Compila ed esegui i benchmark.

## Usage
- `cargo bench` : esegui tutti i benchmark di un pacchetto
- `cargo bench --no-fail-fast` : non interrompere quando un benchmark fallisce
- `cargo bench --no-run` : compila, ma non eseguire i benchmark
- `cargo bench --bench benchmark` : esegui il benchmark specificato
- `cargo bench --profile profilo` : esegui il benchmark con il profilo specificato (predefinito: `bench`)
- `cargo bench --examples` : esegui il benchmark di tutti i target example
- `cargo bench --bins` : esegui il benchmark di tutti i target binari
- `cargo bench --lib` : esegui il benchmark della libreria del pacchetto

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-bench.html
