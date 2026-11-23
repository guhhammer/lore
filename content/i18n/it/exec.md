# exec

Sostituisci il processo corrente con un altro.

## Usage
- `exec comando -con -flag` : sostituisci con il comando specificato utilizzando le variabili di ambiente correnti
- `exec -c comando -con -flag` : sostituisci con il comando specificato utilizzando un ambiente vuoto
- `exec -l comando -con -flag` : sostituisci con il comando specificato ed esegui il login con la shell predefinita
- `exec -a nuovo_nome_processo comando -con -flag` : sostituisci con il comando specificato e cambia il nome del processo

## Links
- More information: https://manned.org/exec.1posix
