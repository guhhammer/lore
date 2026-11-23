# cargo test

Executa os testes unitários e de integração de um pacote Rust.

## Usage
- `cargo t nomedoteste` : executa apenas os testes que contenham um texto específico em seus nomes
- `cargo t -- --test-threads quantidade` : define o número de casos de teste para execução simultânea
- `cargo t -r` : testa os artefatos em modo de publicação (release), com otimizações
- `cargo t --workspace` : testa todos os pacotes no workspace
- `cargo t -p pacote` : executa testes para um pacote específico
- `cargo t -- --nocapture` : executa testes sem ocultar a saída das execuções dos testes

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-test.html
