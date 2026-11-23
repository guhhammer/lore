# cargo

Gestiona proyectos Rust y sus dependencias de módulos (crates). Algunos subcomandos como `build` tienen su propia documentación de uso.

## Usage
- `cargo search cadena_de_busqueda` : busca crates
- `cargo install nombre_crate` : instala un crate binario
- `cargo install --list` : lista los crates binarios instalados
- `cargo init --bin|lib ruta/al/directorio` : crea un nuevo proyecto Rust binario o de biblioteca en el directorio especificado (o en el directorio de trabajo actual por defecto)
- `cargo add dependencia` : añade una dependencia a `Cargo.toml` en el directorio actual
- `cargo b -r` : construye el proyecto Rust en el directorio actual utilizando el perfil de lanzamiento
- `cargo +nightly b` : construye el proyecto Rust en el directorio actual utilizando el compilador nightly (requiere `rustup`)
- `cargo b -j número_de_hilos` : construye usando un número específico de hilos (por defecto es el número de CPUs lógicas)

## Links
- More information: https://doc.rust-lang.org/stable/cargo/
