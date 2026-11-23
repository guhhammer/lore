# sslstrip

Realiza ataques de stripping de Secure Sockets Layer (SSL) basados en el trabajo de Moxie Marlinspike. Realiza un ataque de suplantación de ARP en conjunto.

## Usage
- `sslstrip` : registra sólo el tráfico HTTPS POST en el puerto 10000 por defecto
- `sslstrip --listen=8080` : registra sólo el tráfico HTTPS POST en el puerto 8080
- `sslstrip --ssl --listen=8080` : registra todo el tráfico SSL hacia y desde el servidor en el puerto 8080
- `sslstrip --listen=8080 --all` : registra todo el tráfico SSL y HTTP hacia y desde el servidor en el puerto 8080
- `sslstrip --listen=8080 --write=ruta/al/archivo` : especifica la ruta del archivo para almacenar los registros
- `sslstrip --help` : muestra la ayuda

## Links
- More information: https://www.kali.org/tools/sslstrip/
