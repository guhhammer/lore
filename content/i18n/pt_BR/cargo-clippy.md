# cargo clippy

Conjunto de validadores para identificar erros comuns e melhorar código em Rust.

## Usage
- `cargo clippy` : executa verificações no código no diretório atual
- `cargo clippy --locked` : executa verificações garantindo que o `Cargo.lock` esteja atualizado
- `cargo clippy --workspace` : executa verificações em todos os pacotes no workspace
- `cargo clippy --package pacote` : executa verificações para um pacote específico
- `cargo clippy -- -W clippy::grupo_de_validadores` : executa verificações para um grupo de validadores (veja <https://rust-lang.github.io/rust-clippy/stable/index.html#?groups=cargo,complexity,correctness,deprecated,nursery,pedantic,perf,restriction,style,suspicious>)
- `cargo clippy -- -D warnings` : executa validações tratando avisos como erros
- `cargo clippy -- -A warnings` : executa verificações e ignora avisos
- `cargo clippy --fix` : aplica automaticamente as sugestões do Clippy

## Links
- More information: https://github.com/rust-lang/rust-clippy
