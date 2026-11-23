# alias

Cria um alias -- palavras que são substituídas por um comando. Alias expiram com a sessão da shell atual, a menos que sejam definidos no ficheiro de configuração da shell, por exemplo `~/.bashrc`.

## Usage
- `alias` : lista todos os alias
- `alias palavra="comando"` : cria um alias genérico
- `alias palavra` : visualiza o comando associado a um dado alias
- `unalias palavra` : remove um alias de um comando
- `alias rm="rm -i"` : torna `rm` num comando interativo
- `alias la="ls -a"` : cria `la` como um atalho para `ls -a`

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-alias
