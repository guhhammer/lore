# black

Formatea código Python automáticamente.

## Usage
- `black ruta/al/archivo_o_directorio` : autoformatea un archivo o directorio entero
- `black -c "código"` : formatea el código pasado como cadena
- `black --check ruta/al/archivo_o_directorio` : muestra si un archivo o un directorio sufrirían cambios en caso de ser formateados
- `black --diff ruta/al/archivo_o_directorio` : muestra los cambios que se realizarían en un archivo o directorio sin ejecutarlos (ejecución en seco)
- `black -q ruta/al/archivo_o_directorio` : autoformatea un fichero o directorio, emitiendo exclusivamente mensajes de error a `stderr`
- `black -S ruta/al/archivo_o_directorio` : autoformatea un archivo o directorio sin reemplazar las comillas simples por comillas dobles (ayuda para la adopción, evite usar esto para proyectos nuevos)
