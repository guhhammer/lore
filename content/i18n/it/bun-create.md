# bun create

Crea un nuovo progetto da un template.

## Usage
- `bun c template` : crea un nuovo progetto da un template ufficiale in modo interattivo
- `bun c template percorso/della/destinazione` : crea un nuovo progetto da un template ufficiale in una nuova directory
- `bun c https://github.com/username/repo percorso/della/destinazione` : crea un nuovo progetto da un template di repository GitHub
- `bun c percorso/del/template percorso/della/destinazione` : crea un nuovo progetto da un template locale
- `bun c template percorso/della/destinazione --force` : crea un nuovo progetto sovrascrivendo la directory di destinazione se esiste
- `bun c template percorso/della/destinazione --no-git` : crea un nuovo progetto senza inizializzare automaticamente un repository Git
- `bun c template percorso/della/destinazione --no-install` : crea un nuovo progetto senza installare automaticamente le dipendenze

## Links
- More information: https://bun.com/docs/runtime/templating/create
