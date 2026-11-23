# poetry

Gestiona paquetes y dependencias de Python. Algunos subcomandos, como `about`, `check`, `env`, etc., cuentan con su propia documentación de uso.

## Usage
- `poetry new nombre_del_proyecto` : crea un nuevo proyecto de Poetry en el directorio con un nombre específico
- `poetry add dependencia` : instala y añade una dependencia y sus subdependencias al archivo `pyproject.toml` del directorio actual
- `poetry install` : instala las dependencias del proyecto utilizando el archivo `pyproject.toml` del directorio actual
- `poetry init` : inicializa de forma interactiva (añade `-n` para hacerlo de forma no interactiva) el directorio actual como un nuevo proyecto de Poetry
- `poetry update` : descarga la última versión de todas las dependencias y actualiza `poetry.lock`
- `poetry run comando` : ejecuta un comando dentro del entorno virtual del proyecto
- `poetry version patch|minor|major|prepatch|preminor|premajor|prerelease` : actualiza la versión del proyecto en `pyproject.toml`
- `eval "$(poetry env activate)"` : abre un intérprete de comandos dentro del entorno virtual del proyecto (para versiones anteriores a la 2.0, use `poetry shell`)

## Links
- More information: https://python-poetry.org/docs/cli/
