# as

Ensamblador GNU portátil. Principalmente destinado a ensamblar la salida de `gcc` para ser utilizada por `ld`.

## Usage
- `as ruta/al/archivo.s` : ensambla un archivo, escribiendo la salida en `a.out`
- `as ruta/al/archivo.s -o ruta/al/archivo_salida.o` : ensambla la salida en un archivo específico
- `as -f ruta/al/archivo.s` : genera la salida más rápida omitiendo el preprocesamiento de espacios en blanco y comentarios. (Solo debe usarse con compiladores de confianza)
- `as -I ruta/al/directorio ruta/al/archivo.s` : incluye una ruta específica en la lista de directorios para buscar archivos especificados en las directivas `.include`

## Links
- More information: https://manned.org/as
