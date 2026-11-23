# fd

Uma alternativa para `find`. Visa ser mais rápido e fácil de usar do que `find`.

## Usage
- `fd "padrão|regex"` : encontra recursivamente arquivos que correspondam ao padrão fornecido no diretório atual
- `fd "^foo"` : encontra arquivos que começam com `foo`
- `fd --extension txt` : encontra arquivos com uma extensão específica
- `fd "padrão|regex" caminho/para/diretório` : encontra arquivos em um diretório específico
- `fd --hidden --no-ignore "padrão|regex"` : inclui arquivos ignorados e ocultos na pesquisa
- `fd "padrão|regex" --exec comando` : executa um comando em cada resultado de pesquisa retornado

## Links
- More information: https://github.com/sharkdp/fd#command-line-options
