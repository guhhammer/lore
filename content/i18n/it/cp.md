# cp

Copia file e directory.

## Usage
- `cp percorso/del/file percorso/della/copia` : copia un file in un'altra posizione
- `cp percorso/del/file percorso/della/directory` : copia un file in una directory mantenendo il nome
- `cp -r percorso/della/directory percorso/della/copia` : copia una directory ricorsivamente in un'altra posizione
- `cp -vr percorso/della/directory percorso/della/copia` : copia una directory ricorsivamente in modo verboso (mostra a schermo ogni file copiato)
- `cp -r percorso/della/directory/* percorso/della/seconda/directory` : copia i contenuti di una directory in una seconda directory
- `cp -i *.txt percorso/della/directory` : copia tutti i file di testo in una seconda directory in modo interattivo (chiede conferma prima di sovrascrivere)

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/cp-invocation.html
