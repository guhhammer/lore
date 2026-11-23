# dnf download

Descarga paquetes RPM desde los repositorios DNF. No predeterminado en `dnf` pero admitido a través de `dnf-plugins-core`.

## Usage
- `dnf download paquete` : descarga la última versión de un paquete al directorio actual
- `dnf download paquete --destdir ruta/al/directorio` : descarga un paquete a un directorio específico (el directorio debe existir)
- `dnf download --url paquete` : imprime la URL desde la que se puede descargar el paquete RPM

## Links
- More information: https://dnf-plugins-core.readthedocs.io/en/latest/download.html
