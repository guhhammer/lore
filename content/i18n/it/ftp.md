# ftp

Trasferisci file in modo interattivo tra un server FTP locale e remoto.

## Usage
- `ftp host` : connettiti a un server FTP remoto
- `ftp -A host` : accedi come utente anonimo
- `ftp -n host` : disabilita l'accesso automatico alla connessione iniziale
- `ftp -s:percorso\to\file host` : esegui un file contenente una lista di comandi FTP
- `mget *.png` : [Interattivo] Carica file (espressione `glob`)
- `mput *.zip` : [Interattivo] Carica file (espressione `glob`)
- `mdelete *.txt` : [Interattivo] Cancella file sul server remoto
- `ftp --help` : mostra l'aiuto
