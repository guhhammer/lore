# direnv

Estensione della shell per aggiungere o rimuovere variabili d'ambiente in base alla directory corrente.

## Usage
- `direnv allow .` : permette il caricamento del `.envrc` presente nella directory corrente
- `direnv deny .` : revoca il permesso del `.envrc` presente nella directory corrente
- `direnv edit .` : permette la modifica del `.envrc` nell'editor di testo predefinito, in seguito ricarica l'ambiente
- `direnv reload` : ricarica l'ambiente corrente
- `direnv status` : mostra delle informazioni di debug

## Links
- More information: https://github.com/direnv/direnv/blob/master/man/direnv.1.md
