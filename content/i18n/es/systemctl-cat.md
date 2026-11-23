# systemctl cat

Muestra el contenido completo de los archivos de unidad como los ve systemd.

## Usage
- `systemctl cat unidad` : muestra el contenido y la ruta absoluta de un archivo de unidad
- `systemctl cat unidad1 unidad2 ...` : muestra el contenido de múltiples archivos de unidad
- `systemctl cat plantilla@` : muestra el contenido de un archivo de unidad para una plantilla
- `systemctl cat unidad --user` : muestra el contenido de un archivo de unidad de usuario
