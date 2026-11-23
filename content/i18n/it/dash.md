# dash

Debian Almquist Shell, una implementazione di `sh` moderna, che conforme a POSIX, (non compatibile con Bash).

## Usage
- `dash` : avvia una sessione shell interattiva
- `dash -c "comando"` : esegui un comando, ed esci subito
- `dash percorso/dello/script.sh` : esegui un script
- `dash -x percorso/dello/script.sh` : esegui comandi da un script, stampando ogni comando prima di eseguirlo
- `dash -e percorso/dello/script.sh` : esegui comandi da un script, fermandosi al primo errore
- `dash -s` : leggi ed esegui commandi dal `stdin`

## Links
- More information: https://manned.org/dash
