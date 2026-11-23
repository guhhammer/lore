# dirname

Calcula el directorio padre de una ruta de archivo o directorio.

## Usage
- `dirname ruta/al/archivo_o_directorio` : calcula el directorio padre de una ruta dada
- `dirname ruta/al/archivo_o_directorio1 ruta/al/archivo_o_directorio2 ...` : calcula el directorio padre de múltiples rutas
- `dirname -z ruta/al/archivo_o_directorio1 ruta/al/archivo_o_directorio2 ...` : delimita la salida con un carácter NUL en lugar de una nueva línea (útil cuando se combina con `xargs`)
