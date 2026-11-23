# alias

Cria apelidos -- palavras que são substituídas por um comando. Apelidos expiram ao final da sessão atual do shell de comando, a menos que sejam definidos no arquivo de configuração do shell, por exemplo `~/.bashrc`.

## Usage
- `alias` : lista todos os apelidos
- `alias apelido="comando"` : cria um apelido genérico
- `alias apelido` : visualiza o comando associado a um determinado apelido
- `unalias apelido` : remove um apelido
- `alias rm="rm --interactive"` : torna o comando `rm` interativo
- `alias la="ls --all"` : cria o apelido `la` como um atalho para `ls --all`

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-alias
