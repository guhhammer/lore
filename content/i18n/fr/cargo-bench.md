# cargo bench

Compile et exécute des benchmarks.

## Usage
- `cargo bench` : exécute tous les benchmarks d'un paquet
- `cargo bench --no-fail-fast` : ne pas arréter quand un benchmark échoue
- `cargo bench --no-run` : compile, mais n'exécute pas les benchmarks
- `cargo bench --bench benchmark` : exécute le benchmark spécifié
- `cargo bench --profile profile` : exécute les benchmarks avec un profile spécifique (défaut : `bench`)
- `cargo bench --examples` : exécute les benchmarks sur tous les exemples cibles
- `cargo bench --bins` : exécute les benchmarks sur tous les binaires cibles
- `cargo bench --lib` : exécute les benchmarks sur la bibliothèque du paquet

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-bench.html
