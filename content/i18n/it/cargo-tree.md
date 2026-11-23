# cargo tree

Mostra una visualizzazione ad albero della dipendenza di un progetto. Nota: nell'albero, le dipendenze dei pacchetti contrassegnati con `(*)` sono già state mostrate altrove nel grafo, e quindi non sono ripetute.

## Usage
- `cargo tree` : mostra un albero delle dipendenze del progetto corrente
- `cargo tree --depth n` : mostra solo le dipendenze fino alla profondità specificata (es. quando `n` è 1 mostra solo le dipendenze dirette)
- `cargo tree --prune package_spec` : non visualizzare il pacchetto dato (e le sue dipendenze) nell'albero
- `cargo tree --no-dedupe` : mostra tutte le occorrenze delle dipendenze ripetute
- `cargo tree -e normal|build|dev` : mostra solo dipendenze normali/build/dev

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-tree.html
