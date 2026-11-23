# ftp

Herramientas para interactuar con un servidor a través del Protocolo de Transferencia de Archivos.

## Usage
- `ftp ftp.example.com` : se conecta a un servidor FTP
- `ftp dirección_ip puerto` : se conecta a un servidor FTP especificando su dirección IP y su puerto
- `binary` : [Interactivo] Cambia al modo de transferencia binario (gráficos, archivos comprimidos, etc)
- `prompt off` : [Interactivo] Transfiere varios archivos sin pedir confirmación por cada archivo
- `mget *.png` : [Interactivo] Descarga varios archivos (expresión `glob`)
- `mput *.zip` : [Interactivo] Carga varios archivos (expresión `glob`)
- `mdelete *.txt` : [Interactivo] Elimina varios archivos del servidor remoto
- `rename nombre_archivo_original nuevo_nombre_archivo` : [Interactivo] Cambia el nombre de un archivo en el servidor remoto

## Links
- More information: https://manned.org/ftp
