# alien

Convierte diferentes paquetes de instalación a otros formatos.

## Usage
- `sudo alien -d ruta/al/archivo` : convierte un archivo de instalación específico al formato Debian (extensión `.deb`)
- `sudo alien -r ruta/al/archivo` : convierte un archivo de instalación específico al formato Red Hat (extensión `.rpm`)
- `sudo alien -t ruta/al/archivo` : convierte un archivo de instalación específico al formato de instalación de Slackware (extensión `.tgz`)
- `sudo alien -d -i ruta/al/archivo` : convierte un archivo de instalación específico al formato Debian y lo instala en el sistema

## Links
- More information: https://manned.org/alien
