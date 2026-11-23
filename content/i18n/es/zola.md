# zola

Un generador de sitios estáticos en un único binario con todo incorporado.

## Usage
- `zola init mi_sitio` : crea la estructura de directorios utilizada por Zola en el directorio dado
- `zola build` : construye todo el sitio en el directorio `public` después de eliminarlo
- `zola build --output-dir ruta/al/directorio_resultado/` : construye todo el sitio en un directorio diferente
- `zola serve` : construye y sirve el sitio usando un servidor local (por defecto es `127.0.0.1:1111`)
- `zola check` : construye todas las páginas como lo haría el comando build, pero sin escribir ninguno de los resultados al disco

## Links
- More information: https://www.getzola.org/documentation/getting-started/cli-usage/
