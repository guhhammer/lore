# snakefmt

Formata arquivos Snakemake.

## Usage
- `snakefmt caminho/para/snakefile` : formata um Snakefile específico
- `snakefmt caminho/para/diretorio` : formata todos os Snakefiles recursivamente em uma pasta específica
- `snakefmt --config caminho/para/config.toml caminho/para/snakefile` : formata um arquivo usando um arquivo de configuração específico
- `snakefmt --line-length 100 caminho/para/snakefile` : formata um arquivo usando um comprimento máximo de linha específico
- `snakefmt --diff caminho/para/snakefile` : exibe às mudanças que seriam realizadas sem realiza-las

## Links
- More information: https://github.com/snakemake/snakefmt#usage
