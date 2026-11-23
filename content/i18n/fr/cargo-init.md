# cargo init

Crée un nouveau paquet Cargo. Équivalent de `cargo new`, mais spécifier un répertoire est optionnel.

## Usage
- `cargo init` : initialise un projet Rust binaire dans le répertoire actuel
- `cargo init chemin/vers/répertoire` : initailise un projet de binaire Rust dans le répertoire spécifié
- `cargo init --lib` : initialise un projet de bibliothèque Rust dans le répertoire spécifié
- `cargo init --vcs git|hg|pijul|fossil|none` : initialise un dépôt de système de gestion de version dans le répertoire du projet (défaut : `git`)
- `cargo init --name nom_du_paquet` : défini le nom du paquet (défaut : nom du répertoire)

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-init.html
