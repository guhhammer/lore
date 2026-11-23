# git clean

Elimina i file non tracciati dall'albero di lavoro.

## Usage
- `git clean` : elimina i file che non sono tracciati da Git
- `git clean -i` : elimina in modo interattivo i file non tracciati da Git
- `git clean -n` : mostra quali file non tracciati sarebbero eliminati, senza però eliminarli davvero
- `git clean -f` : forza l'eliminazione dei file non tracciati da Git
- `git clean -f -d` : forza l'eliminazione delle directory non tracciate da Git
- `git clean -x` : elimina i file non tracciati, compresi quelli da ignorare elencati in `.gitignore` e `.git/info/exclude`

## Links
- More information: https://git-scm.com/docs/git-clean
