# cargo owner

Gestisci i maintainer (owner) di un crate sul registro.

## Usage
- `cargo owner -a username|github:org_name:team_name crate` : invita l'utente o il team indicato come owner
- `cargo owner -r username|github:org_name:team_name crate` : rimuovi l'utente o il team indicato come owner
- `cargo owner -l crate` : elenca gli owner di un crate
- `cargo owner --registry nome` : usa il registro specificato (i nomi dei registri possono essere definiti nella configurazione - il predefinito è <https://crates.io>)

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-owner.html
