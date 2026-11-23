# pgrep

Busca o envía señales a procesos por su nombre.

## Usage
- `pgrep nombre_del_proceso` : devuelve los PID de cualquier proceso en ejecución con una cadena de comando coincidente
- `pgrep -a nombre_del_proceso` : también muestra el comando completo
- `pgrep -f "nombre_del_proceso parámetro"` : busca procesos incluyendo sus opciones de línea de comandos
- `pgrep -u usuario nombre_del_proceso` : busca procesos ejecutados por un usuario específico

## Links
- More information: https://manned.org/pgrep
