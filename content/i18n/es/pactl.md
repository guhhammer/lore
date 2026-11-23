# pactl

Controla un servidor de sonido PulseAudio en ejecución.

## Usage
- `pactl info` : muestra información sobre el servidor de sonido
- `pactl list sinks short` : lista todos los sinks (u otros tipos - los sinks son salidas y los sink-inputs son flujos de audio activos)
- `pactl set-default-sink 1` : cambia el sink (salida) predeterminado a 1 (el número se puede obtener mediante el subcomando `list`)
- `pactl move-sink-input 627 1` : mueve el sink-input 627 al sink 1
- `pactl set-sink-volume 1 0.75` : establece el volumen del sink 1 al 75%
- `pactl set-sink-mute @DEFAULT_SINK@ toggle` : cambia el estado de silencio del sink predeterminado (usando el nombre especial `@DEFAULT_SINK@`)

## Links
- More information: https://manned.org/pactl
