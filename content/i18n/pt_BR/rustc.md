# rustc

O compilador Rust. Projetos Rust geralmente usam o `cargo` em vez de chamar `rustc` diretamente.

## Usage
- `rustc caminho/para/arquivo.rs` : compila uma crate binária
- `rustc -C lto -C opt-level=0|1|2|3|s|z caminho/para/arquivo.rs` : compila com otimizações (s significa otimizar o tamanho do binário; z é o mesmo com ainda mais otimizações)
- `rustc -g caminho/para/arquivo.rs` : compila com informações de depuração
- `rustc --explain código_de_erro` : explica uma mensagem de erro
- `rustc -C target-cpu=native caminho/para/arquivo.rs` : compila com otimizações específicas de arquitetura para a CPU atual
- `rustc --print target-list` : exibe lista de targets
- `rustc --target target_triplo caminho/para/arquivo.rs` : compila para um target específico

## Links
- More information: https://doc.rust-lang.org/stable/rustc/
