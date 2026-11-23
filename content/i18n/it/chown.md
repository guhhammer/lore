# chown

Cambia la proprietà di utente e gruppo di file e directory.

## Usage
- `sudo chown utente percorso/del/file_o_directory` : cambia l'utente proprietario di un file/directory
- `sudo chown utente:gruppo percorso/del/file_o_directory` : cambia l'utente proprietario e il gruppo di un file/directory
- `sudo chown utente: percorso/del/file_o_directory` : cambia sia l'utente proprietario che il gruppo in entrambi i casi con lo stesso nome `utente`
- `chown :gruppo percorso/del/file_o_directory` : cambia il gruppo di un file in un gruppo a cui l'utente corrente appartiene
- `sudo chown -R utente percorso/della/directory` : cambia ricorsivamente il proprietario di una directory e dei suoi contenuti
- `sudo chown -h utente percorso/del/link_simbolico` : cambia il proprietario di un link simbolico
- `sudo chown --reference percorso/del/file_riferimento percorso/del/file_o_directory` : cambia il proprietario di un file/directory in modo che coincida con un file di riferimento
