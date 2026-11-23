# compgen

Um programa para auto completar comandos no Bash, ele é executado ao pressionar duas vezes a tecla `<Tab>`.

## Usage
- `compgen -c` : exibe todos os comandos que você pode executar
- `compgen -a` : exibe todos os alias
- `compgen -A function` : exibe todas as funções que você pode executar
- `compgen -k` : exibe todas as palavras reservadas do shell
- `compgen -ac ls` : exibe todos os comandos/alias que iniciam com o termo 'ls'

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-compgen
