# ack

Una herramienta de búsqueda como grep, optimizada para desarrolladores.

## Usage
- `ack "patrón_de_búsqueda"` : busca archivos que contengan una cadena o `regex` en el directorio actual de forma recursiva
- `ack -i "patrón_de_búsqueda"` : busca un patrón sin distinción entre mayúsculas y minúsculas
- `ack -o "patrón_de_búsqueda"` : busca líneas que coincidan con un patrón, imprimiendo solamente el texto coincidente y no el resto de la línea
- `ack -t ruby "patrón_de_búsqueda"` : limita la búsqueda a archivos de un tipo específico
- `ack -t noruby "patrón_de_búsqueda"` : busca archivos que no sean de un cierto tipo
- `ack -c -h "patrón_de_búsqueda"` : cuenta el número total de coincidencias encontradas
- `ack -c -l "patrón_de_búsqueda"` : imprime solo los nombres de los archivos y el número de coincidencias de cada archivo
- `ack --help-types` : lista todos los valores que se pueden utilizar con `--type`

## Links
- More information: https://beyondgrep.com/documentation/
