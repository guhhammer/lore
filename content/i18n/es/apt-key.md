# apt-key

Herramienta para la gestión de claves para el Gestor de Paquetes APT (APT Package Manager) en Debian y Ubuntu.

## Usage
- `apt-key list` : muestra las claves de confianza
- `apt-key add archivo_clave_pública.asc` : añade una clave al almacén de claves de confianza
- `apt-key del identificador_de_clave` : borra una clave del almacén de claves de confianza
- `wget -qO - https://host.tld/archivo.clave | apt-key add -` : añade un clave remota al almacén de claves de confianza
- `apt-key adv --keyserver pgp.mit.edu --recv identificador_de_clave` : añade una clave de un servidor de claves con el identificador de la clave

## Links
- More information: https://manned.org/apt-key
