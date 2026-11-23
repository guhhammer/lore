# distrobox-export

Exporta aplicaciones/servicios/binarios del contenedor al sistema operativo anfitrión.

## Usage
- `distrobox-export -a paquete -ef "--foreground"` : exporta una aplicación del contenedor al anfitrión (la entrada de escritorio/ícono aparecerá en la lista de aplicaciones del sistema anfitrión)
- `distrobox-export -b ruta/al/binario -ep ruta/al/binario_en_anfitrión` : exporta un binario del contenedor al anfitrión
- `distrobox-export -b ruta/al/binario -ep ruta/a/exportar` : exporta un binario del contenedor al anfitrión (es decir, `$HOME/.local/bin`)
- `distrobox-export --service paquete -ef "--allow-newer-config" -S` : exporta un servicio desde el contenedor al anfitrión (`--sudo` ejecutará el servicio como root dentro del contenedor)
- `distrobox-export -a paquete -d` : elimina o deja de exportar una aplicación exportada

## Links
- More information: https://distrobox.it/usage/distrobox-export/
