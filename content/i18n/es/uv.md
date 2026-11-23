# uv

Un rápido gestor de paquetes y proyectos Python. Algunos subcomandos como `tool` y `python` tienen su propia documentación de uso.

## Usage
- `uv init` : crea un nuevo proyecto Python en el directorio actual
- `uv init ruta/al/directorio` : crea un nuevo proyecto Python en la ruta especificada
- `uv add paquete` : añade una nueva dependencia al proyecto
- `uv remove paquete` : elimina una dependencia del proyecto
- `uv run ruta/al/script.py` : ejecuta un script en el entorno del proyecto
- `uv run comando` : ejecuta un comando en el entorno del proyecto
- `uv sync` : actualiza el entorno de un proyecto desde `pyproject.toml`
- `uv lock` : crea un archivo de bloqueo para las dependencias del proyecto

## Links
- More information: https://docs.astral.sh/uv/reference/cli/
