# filefrag

Informa del grado de fragmentación de un archivo en particular.

## Usage
- `filefrag ruta/al/archivo1 ruta/al/archivo2 ...` : muestra un informe para uno o más archivos
- `filefrag -k ruta/al/archivo` : muestra un informe con un tamaño de bloque de 1024 bytes
- `filefrag -b1024|1K|1M|1G|... ruta/al/archivo` : muestra un informe con un tamaño de bloque determinado
- `filefrag -s ruta/al/archivo1 ruta/al/archivo2 ...` : sincroniza el archivo antes de solicitar la asignación
- `filefrag -x ruta/al/archivo1 ruta/al/archivo2 ...` : muestra la asignación de atributos extendidos
- `filefrag -v ruta/al/archivo1 ruta/al/archivo2 ...` : muestra un informe con información detallada

## Links
- More information: https://manned.org/filefrag
