# cargo build

Compile un paquet local et toutes ses dépendances.

## Usage
- `cargo b` : compile un ou plusieurs paquets définis dans le manifeste `Cargo.toml` dans le dossier local
- `cargo b -r` : compile les artefacts avec le mode publication, avec des optimisations
- `cargo b --locked` : le fichier `Cargo.lock` doit être à jour
- `cargo b --workspace` : compile tous les paquets de l'espace de travail
- `cargo b -p paquet` : compile un paquet spécifique
- `cargo b --bin nom_du_binaire` : compile uniquement le binaire spécifié
- `cargo b --test nom_du_test` : compile uniquement le test cible spécifié

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-build.html
