# axel

Acceleratore di download. Supporta HTTP, HTTPS, FTP e FTPS.

## Usage
- `axel url` : scarica un URL in un file
- `axel url -o percorso/del/file` : scarica e specifica un file di output
- `axel -n number url` : scarica con un numero specifico di connessioni
- `axel -Snumber url` : usa un numero specifico di mirror per la ricerca e il download
- `axel -s speed url` : limita la velocità di download (byte al secondo)
- `axel -4 url` : usa solo il protocollo IPv4 quando si connette all'host
- `axel -q -U "Mozilla/5.0" url` : limita l'output a `stdout` e usa un user-agent personalizzato durante il download

## Links
- More information: https://manned.org/axel
