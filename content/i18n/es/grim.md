# grim

Obtiene imágenes (capturas de pantalla) de un compositor Wayland.

## Usage
- `grim` : hace una captura de pantalla
- `grim -o DP-1 ruta/al/archivo_resultado` : captura de pantalla a un archivo específico
- `grim -g "posición_x,posición_y anchoxalto"` : captura de pantalla de una región específica
- `grim -g "$(slurp)"` : selecciona una región específica y toma una captura de dicha porción, usando slurp
- `grim "ruta/al/archivo.png"` : utiliza un nombre de archivo personalizado
- `grim - | clipboard_manager` : captura de pantalla y la copia al portapapeles

## Links
- More information: https://manned.org/grim
