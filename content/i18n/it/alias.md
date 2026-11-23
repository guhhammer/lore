# alias

Crea alias - parole sostituite da stringhe di comandi. Gli alias scadono con la sessione shell corrente a meno che non siano definiti nel file di configurazione della shell, es. `~/.bashrc` per Bash o `~/.zshrc` per Zsh.

## Usage
- `alias` : elenca tutti gli alias
- `alias parola="comando"` : crea un alias generico
- `alias parola` : visualizza il comando associato a un dato alias
- `unalias parola` : rimuovi un comando alias
- `alias rm="rm --interactive"` : rendi `rm` interattivo
- `alias la="ls --all"` : crea `la` come scorciatoia per `ls --all`

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-alias
