# deborphan

Muestra paquetes huérfanos en sistemas operativos usando el administrador de paquetes APT.

## Usage
- `deborphan` : muestra paquetes de biblioteca (de la sección "libs" del repositorio de paquetes) que no son requeridos por otro paquete
- `deborphan --guess-all` : lista paquetes huérfanos de la sección "libs", así como paquetes huérfanos que tienen un nombre que parece un nombre de biblioteca
- `deborphan -n` : busca paquetes que son recomendados o sugeridos (pero no requeridos) por otro paquete

## Links
- More information: https://manned.org/deborphan
