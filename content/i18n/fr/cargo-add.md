# cargo add

Ajoute des dépendences au manifeste `Cargo.toml` d'un projet Rust.

## Usage
- `cargo add dépendance` : ajoute la dernière version d'une dépendance au projet actuel
- `cargo add dépendance@version` : ajoute une version spécifique d'une dépendance
- `cargo add dépendance -F fonctionnalité_1,fonctionnalité_2,...` : ajoute une dépendance et active une ou plusieures fonctionnalités
- `cargo add dépendance --optional` : ajoute une dépendance optionnelle, qui sera exposée comme une fonctionnalité du crate
- `cargo add --path chemin/vers/crate` : ajoute un crate local en tant que dépendance
- `cargo add dépendance --dev|build` : ajoute une dépendance de développement ou de compilation
- `cargo add dépendance --no-default-features` : ajoute une dépendance avec toutes les fonctionnalités par défaut désactivées

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-add.html
