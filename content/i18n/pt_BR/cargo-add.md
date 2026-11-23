# cargo add

Adiciona dependências ao arquivo `Cargo.toml` de um projeto Rust.

## Usage
- `cargo add dependência` : adiciona a versão mais recente de uma dependência ao projeto atual
- `cargo add dependência@versão` : adiciona uma versão específica de uma dependência
- `cargo add dependência -F funcionalidade_1,funcionalidade_2,...` : adiciona uma dependência e habilita uma ou mais funcionalidades específicas
- `cargo add dependência --optional` : adiciona uma dependência opcional, que será exposta como uma funcionalidade da crate
- `cargo add --path caminho/para/crate` : adiciona uma crate local como dependência
- `cargo add dependência --dev|build` : adiciona uma dependência de desenvolvimento ou de compilação
- `cargo add dependência --no-default-features` : adiciona uma dependência com todas as funcionalidades padrão desabilitadas

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-add.html
