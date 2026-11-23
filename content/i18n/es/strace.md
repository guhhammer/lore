# strace

Herramienta de diagnóstico para rastrear llamadas al sistema.

## Usage
- `sudo strace -p pid` : rastrea las llamadas de un proceso usando su PID
- `sudo strace -p pid -e llamada_sis,llamada_sis2,...` : rastrea un proceso y filtra la salida según expresiones de llamadas al sistema
- `sudo strace -p pid -c` : cuenta el tiempo, número de llamada y errores para cada llamada al sistema y devuelve un resumen cuando el programa acabe
- `sudo strace -p pid -T -s 32` : muestra el tiempo dedicado en cada llamada al sistema y especifica el tamaño máximo de cadena a imprimir
- `strace programa` : rastrea un programa ejecutándolo
- `strace -e trace=file programa` : rastrea las operaciones de archivos de un programa
- `strace -f -e trace=network -o rastreo.txt programa` : rastrea las operaciones de red de un programa, así como las de todos sus procesos hijos, guardando la salida en un archivo

## Links
- More information: https://manned.org/strace
