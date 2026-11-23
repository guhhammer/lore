# uv tree

Muestra las dependencias del proyecto en formato de árbol.

## Usage
- `uv tree` : muestra el árbol de dependencias del entorno actual
- `uv tree --universal` : muestra el árbol de dependencias para todos los entornos
- `uv tree -d n` : muestra el árbol de dependencias hasta una determinada profundidad
- `uv tree --outdated` : muestra la última versión disponible para todos los paquetes obsoletos
- `uv tree --no-dev` : excluye dependencias del grupo dev
- `uv tree --invert` : muestra el árbol invertido, para que los hijos sean dependientes en lugar de dependencias

## Links
- More information: https://docs.astral.sh/uv/reference/cli/#uv-tree
