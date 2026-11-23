# smbclient

Programma client simile ad FTP per server SMB/CIFS.

## Usage
- `smbclient //server/share` : connettiti ad una share (all'utente verrà richiesta la password; `exit` per uscire dalla sessione)
- `smbclient //server/share -U username` : connettiti con un altro nome utente
- `smbclient //server/share -W dominio -U username` : connettiti con un altro gruppo di lavoro
- `smbclient //server/share -U username%password` : connettiti con un nome utente ed una password
- `smbclient //server/share -D percorso/della/directory -c "get file.txt"` : scarica un file dal server
- `smbclient //server/share -D percorso/della/directory -c "put file.txt"` : carica un file sul server
- `smbclient -L server --no-pass` : elenca le share di un server in modo anonimo

## Links
- More information: https://manned.org/smbclient
