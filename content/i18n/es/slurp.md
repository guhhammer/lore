# slurp

Selecciona una región en un compositor Wayland.

## Usage
- `slurp` : selecciona una región y la imprime en `stdout`
- `slurp -d` : selecciona una región e imprímela en `stdout`, mientras muestras las dimensiones de la selección
- `slurp -p` : selecciona un único punto en lugar de una región
- `slurp -o -f '%o'` : selecciona una salida e imprime su nombre
- `grim -g "$(slurp -w 0)"` : selecciona una región determinada y hace una captura de pantalla sin bordes utilizando `grim`
- `wf-recorder -g "$(slurp -w 0)"` : selecciona una región determinada y graba un vídeo sin bordes utilizando `wf-recorder`

## Links
- More information: https://github.com/emersion/slurp/blob/master/slurp.1.scd
