# cargo

Gerencia projetos Rust e as dependências dos modulos (crates). Alguns subcomandos como `build` tem a sua própria documentação.

## Usage
- `cargo search string_procurada` : procura por crates
- `cargo install nome_da_crate` : instala uma crate
- `cargo install --list` : lista as crates instaladas
- `cargo init --bin|lib caminho/para/diretório` : cria um novo binário ou projeyo Rust de biblioteca no diretório especificado (ou o diretório atual por padrão)
- `cargo add dependência` : adiciona uma dependência ao Cargo.toml no diretório atual
- `cargo b -r` : constrói o projeto Rust no diretório atual usando o perfil de lançamento
- `cargo +nightly b` : constrói o projeto Rust no diretório atual utilizando o nightly compilador
- `cargo b -j número_de_threads` : constrói o projeto Rust utilizando um número específico de threads (padrão é o número de cores do CPU)

## Links
- More information: https://doc.rust-lang.org/stable/cargo/
