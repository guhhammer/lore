# cargo

Gestion d'un projet Rust et ses dépendences (crates). Certaines sous-commandes comme `build` ont leur propre documentation.

## Usage
- `cargo search recherche` : recherche des crates
- `cargo install nom_du_crate` : installe un crate
- `cargo install --list` : liste les crates déjà installés
- `cargo init --bin|lib chemin/vers/répertoire` : crée un nouveau binaire ou librairie du projet Rust dans les répertoires spécifiés (ou dans le répertoire actuel par défaut)
- `cargo add dépendance` : ajoute une dépendance à `Cargo.toml` dans le répertoire actuel
- `cargo b -r` : compile le projet Rust dans le répertoire actuel en utilisant le profil release
- `cargo r` : exécute le binaire compilé (le compile s’il ne l’est pas déjà)
- `cargo +nightly b` : compile le projet Rust dans le répertoire actuel en utilisant le compilateur nightly

## Links
- More information: https://doc.rust-lang.org/stable/cargo/
