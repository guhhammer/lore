# screen

Mantiene una sesión abierta en un servidor remoto. Gestiona varias ventanas con una única conexión SSH.

## Usage
- `screen` : inicia una nueva sesión de pantalla
- `screen -S nombre_de_sesión` : inicia una nueva sesión de pantalla con nombre
- `screen -dmLS nombre_de_sesión comando` : inicia un nuevo programa residente y registra la salida en `screenlog.x`
- `screen -ls` : muestra las sesiones de pantalla abiertas
- `screen -r nombre_de_sesión` : vuelve a conectarse a una pantalla abierta
- `<Ctrl a><d>` : se desvincula desde dentro de una pantalla
- `<Ctrl a><k>` : finaliza la sesión de pantalla actual
- `screen -X -S nombre_de_sesión quit` : desactiva una pantalla desacoplada

## Links
- More information: https://manned.org/screen
