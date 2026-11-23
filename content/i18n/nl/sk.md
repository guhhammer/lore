# sk

Fuzzy finder geschreven in Rust.

## Usage
- `find pad/naar/map -type f | sk` : start `skim` op alle bestanden in de opgegeven map
- `ps aux | sk` : start `skim` voor actieve processen
- `sk --query "query"` : start `skim` met een bepaalde query
- `find pad/naar/map -type f | sk --multi >pad/naar/bestand` : selecteer meerdere bestanden met `<Shift Tab>` en schrijf naar een bestand

## Links
- More information: https://github.com/skim-rs/skim
