# cargo

Verwalte Rust-Projekte und deren Abhängigkeiten (crates). Manche Unterbefehle wie `build` sind separat dokumentiert.

## Usage
- `cargo search suche` : suche nach Abhängigkeiten (crates)
- `cargo install abhängigkeit` : installiere eine Abhängigkeit (crate)
- `cargo install --list` : liste alle installierten Abhängigkeiten (crates) auf
- `cargo init --bin|lib` : erzeuge ein neues Rust-Projekt als Anwendung oder Bibliothek im aktuellen Verzeichnis
- `cargo new pfad/zu/verzeichnis --bin|lib` : erzeuge ein neues Rust-Projekt als Anwendung oder Bibliothek im angegebenen Verzeichnis
- `cargo b` : erstelle (bzw. kompiliere) ein Rust-Projekt im aktuellen Verzeichnis
- `cargo b -j thread_anzahl` : erstelle (bzw. kompiliere) ein Rust-Projekt mit einer bestimmten Anzahl an Threads (standardmäßig die Anzahl der CPU-Kerne)

## Links
- More information: https://doc.rust-lang.org/stable/cargo/
