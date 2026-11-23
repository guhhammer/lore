# aria2c

Utilità di download veloce multiprotocollo. Supporta HTTP(S), FTP, SFTP, BitTorrent e Metalink.

## Usage
- `aria2c "url"` : scarica un URI specifico in un file
- `aria2c -o percorso/del/file "url"` : scarica un file da un URI con nome di output specifico
- `aria2c -Z "url1" "url2" ...` : scarica più file diversi in parallelo
- `aria2c --checksum sha-256=hash "url1" "url2" ...` : scarica lo stesso file da diversi mirror verificando il checksum
- `aria2c -i percorso/del/file -j numero_download` : scarica gli URI elencati in un file con numero specifico di download paralleli
- `aria2c -s numero_connessioni "url"` : scarica con più connessioni
- `aria2c --ftp-user nome_utente --ftp-passwd password "url"` : download FTP con username e password
- `aria2c --max-download-limit velocità "url"` : limita la velocità di download in byte/s

## Links
- More information: https://aria2.github.io/manual/en/html/aria2c.html
