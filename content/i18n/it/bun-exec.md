# bun exec

Esegui comandi shell o file di script usando il runtime di Bun. Nota: quando esegui dalla shell, ricorda di sfuggire alle virgolette.

## Usage
- `bun exec "echo hello"` : esegui un comando semplice
- `bun exec "ls -la"` : esegui un comando con flag
- `bun exec "echo \"hello friends\""` : esegui un comando contenente virgolette
- `bun exec "mkdir test && cd test"` : esegui un comando shell combinato
- `bun exec percorso/dello/script` : esegui un file di script

## Links
- More information: https://bun.com/docs/runtime/shell
