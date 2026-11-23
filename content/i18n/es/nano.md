# nano

Editor sencillo y fácil de usar. Un clon libre y mejorado de `Pico`.

## Usage
- `nano` : inicia el editor
- `nano -I` : inicia el editor sin usar archivos de configuración
- `nano ruta/al/archivo1 ruta/al/archivo2 ...` : abre archivos específicos, moviéndose al siguiente archivo cuando se cierra el anterior
- `nano +línea,columna ruta/al/archivo` : abre un archivo específico, posicionando el cursor en la línea y columna específica
- `nano -S ruta/al/archivo` : abre un archivo específico y activa el ajuste de línea suave (wrapping)
- `nano -i ruta/al/archivo` : abre un archivo específico y sangra nuevas líneas al nivel de las líneas anteriores
- `nano -B ruta/al/archivo` : abre nano y crea un archivo de respaldo (`ruta/al/archivo~`) cuando se guardan las ediciones

## Links
- More information: https://nano-editor.org/dist/latest/nano.html
