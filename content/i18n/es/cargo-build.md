# cargo build

Compila un paquete local y todas sus dependencias.

## Usage
- `cargo b` : construye el paquete o los paquetes definidos por el archivo manifiesto `Cargo.toml` en la ruta local
- `cargo b -r` : construye artefactos en modo de lanzamiento, con optimizaciones
- `cargo b --locked` : exige que `Cargo.lock` esté actualizado
- `cargo b --workspace` : construye todos los paquetes en el espacio de trabajo
- `cargo b -p paquete` : construye un paquete determinado
- `cargo b --bin nombre` : construye solo el binario especificado
- `cargo b --test nombre_de_la_prueba` : construye solamente el objetivo de prueba especificado

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-build.html
