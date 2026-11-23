# stdbuf

Ejecuta un comando con operaciones de almacenamiento en búfer modificadas para sus flujos estándar.

## Usage
- `stdbuf -i 512K comando` : cambia el tamaño del búfer de `stdin` a 512 KiB
- `stdbuf -o L comando` : cambia el búfer de `stdout` a búfer por líneas
- `stdbuf -e 0 comando` : configura el búfer `stderr` sin almacenamiento en búfer
