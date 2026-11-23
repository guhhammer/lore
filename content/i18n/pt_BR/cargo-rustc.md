# cargo rustc

Compila um pacote Rust.

## Usage
- `cargo rustc -- rustc_options` : compila o pacote e passa opções para `rustc`
- `cargo rustc -r` : compila os artefatos em modo de publicação (release), com otimizações
- `cargo rustc -r -- -C target-cpu=native` : compila com otimizações específicas para a arquitetura do CPU atual
- `cargo rustc -- -C opt-level=1|2|3` : compila com otimização de velocidade
- `cargo rustc -- -C opt-level=s|z` : compila com otimização de tamanho (`z` também desativa a vetorização de ciclos)
- `cargo rustc --lib -- -D unsafe-code` : verifica se o pacote usa código com padrões inseguros de acesso à memória
- `cargo rustc -p pacote` : compila um pacote específico
- `cargo rustc --bin nome` : compila apenas o binário especificado

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-rustc.html
