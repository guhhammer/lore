# cargo build

Compila um projeto Rust em um pacote local incluindo todas as suas dependências.

## Usage
- `cargo b` : compila o pacote ou pacotes definidos pelo arquivo `Cargo.toml` no diretório local
- `cargo b -r` : compila os artefatos em modo de publicação (release), com otimizações
- `cargo b --locked` : compila um pacote garantindo que o `Cargo.lock` esteja atualizado
- `cargo b --workspace` : compila todos os pacotes no workspace
- `cargo b -p pacote` : compila um pacote específico
- `cargo b --bin nome` : compila apenas o binário especificado
- `cargo b --test nome_do_teste` : compila apenas um teste específico

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-build.html
