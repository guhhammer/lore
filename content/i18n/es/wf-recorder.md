# wf-recorder

Screencast para Wayland opcionalmente con audio. Por defecto necesita terminar el proceso con `<Ctrl c>`.

## Usage
- `wf-recorder -f salida.mp4` : grabación de un archivo MP4
- `wf-recorder -a -f /ruta/al/archivo_con_audio.webm` : graba video incluyendo audio; y esto incluye acceso al micrófono y los sonidos del sistema
- `wf-recorder -g "$(slurp)"` : selecciona y graba una porción de la pantalla utilizando `slurp`, guardando en `recording.mp4` de forma predeterminada

## Links
- More information: https://github.com/ammen99/wf-recorder
