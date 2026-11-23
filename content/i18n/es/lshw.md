# lshw

Muestra información detallada sobre la configuración del hardware como usuario root.

## Usage
- `sudo lshw -X` : inicia la interfaz gráfica X11 (si está disponible)
- `sudo lshw -short` : muestra todo el hardware en formato tabular
- `sudo lshw -c disk -c storage -short` : muestra varias clases de hardware (todos los discos y controladores de almacenamiento) en formato tabular
- `sudo lshw -c network -html|xml|json >interfaces.html|.xml|.json` : guarda todas las interfaces de red en un archivo HTML/XML/JSON
- `sudo lshw -c network -sanitize` : muestra las interfaces de red sin revelar información confidencial (direcciones IP, números de serie, etc.)
- `sudo lshw -c system|bridge|memory|processor|address|storage|disk|tape|bus|network|display|input|printer|multimedia|communication|power|volume|generic` : muestra una clase concreta de hardware

## Links
- More information: https://ezix.org/project/wiki/HardwareLiSter
