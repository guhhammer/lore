# cargo yank

Rimuovi un crate pubblicato dall'indice. Questo dovrebbe essere usato solo quando hai pubblicato accidentalmente una versione gravemente difettosa. Nota: questo non rimuove alcun dato. Il crate rimane disponibile dopo uno yank — questo impedisce solo che nuovi progetti lo usino.

## Usage
- `cargo yank crate@versione` : rimuovi la versione specificata di un crate dall'indice
- `cargo yank --undo crate@versione` : annulla uno yank (cioè permetti di scaricarlo di nuovo)
- `cargo yank --registry nome crate@versione` : usa il registro specificato (i nomi dei registri possono essere definiti nella configurazione - il predefinito è <https://crates.io>)

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-yank.html
