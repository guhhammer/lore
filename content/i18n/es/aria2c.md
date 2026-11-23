# aria2c

Utilidad de descarga rápida. Soporta HTTP(S), FTP, SFTP, BitTorrent y Metalink.

## Usage
- `aria2c "url"` : descarga un URI específico a un archivo
- `aria2c -o ruta/al/archivo "url"` : descarga un archivo de una URI con un nombre de salida específico
- `aria2c -Z "url1" "url2" ...` : descarga varios archivos diferentes en paralelo
- `aria2c --checksum sha-256=suma_de_comprobación "url1" "url2" ...` : descarga el mismo archivo desde diferentes espejos y verifica la suma de comprobación del archivo descargado
- `aria2c -i ruta/al/archivo -j número_de_descargas` : descarga las URI enumeradas en un archivo con un número determinado de descargas paralelas
- `aria2c -s número_de_conexiones "url"` : descarga con varias conexiones
- `aria2c --ftp-user usuario --ftp-passwd contraseña "url"` : descarga a través de FTP con un nombre de usuario y contraseña
- `aria2c --max-download-limit velocidad "url"` : limita la velocidad de descarga en bytes por segundo

## Links
- More information: https://aria2.github.io/manual/en/html/aria2c.html
