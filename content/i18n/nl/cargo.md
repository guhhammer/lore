# cargo

Beheer Rust-projecten en hun module-afhankelijkheden (crates). Sommige subcommando's zoals `build` hebben hun eigen documentatie.

## Usage
- `cargo search zoekopdracht` : zoek naar crates
- `cargo install crate_naam` : installeer een crate
- `cargo install --list` : geef een lijst van geïnstalleerde crates
- `cargo init --bin|lib pad/naar/map` : maak een nieuw Rust-project van het type binary (bin) of bibliotheek (lib) in de gegeven map (standaard de huidige map)
- `cargo add afhankelijkheid` : voeg een afhankelijkheid toe aan `Cargo.toml` in de huidige map
- `cargo b -r` : bouw het Rust-project in de huidige map door gebruik te maken van het release-profiel
- `cargo r` : voer de gecompileerde binary uit (wordt eerst gecompileerd als het nog niet is gecompileerd)
- `cargo +nightly b` : bouw het Rust-project in de huidige map door gebruik te maken van de nightly compiler (vereist `rustup`)

## Links
- More information: https://doc.rust-lang.org/stable/cargo/
