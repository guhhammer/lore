# bacon

Un verificador de código en segundo plano para Rust.

## Usage
- `bacon` : ejecuta `cargo check` cada vez que se detecta un cambio en el directorio actual
- `bacon test ruta/al/directorio` : ejecuta `cargo test` siempre que se detecte un cambio en el directorio dado
- `bacon check-all` : ejecuta `cargo check` contra todos los objetivos siempre que se detecte un cambio en el directorio actual
- `bacon run|test|clippy|doc|...` : ejecuta un trabajo específico cada vez que se detecta un cambio en el directorio actual
- `bacon --list-jobs` : lista todos los trabajos disponibles
- `bacon --init` : inicializa un archivo de configuración `bacon.toml` en el directorio actual

## Links
- More information: https://github.com/Canop/bacon
