# ruff format

Un formador de código Python extremadamente rápido. Si no se especifican archivos o directorios, se utiliza el directorio de trabajo actual de forma predeterminada.

## Usage
- `ruff format ruta/al/archivo_o_directorio1 ruta/al/archivo_o_directorio2 ...` : formatea los archivos o directorios
- `ruff format --check` : imprime qué archivos habrían sido modificados y devuelve un código de salida no cero si hay archivos a reformatear; y cero de lo contrario
- `ruff format --diff` : imprime qué cambios se harían sin modificar los archivos

## Links
- More information: https://docs.astral.sh/ruff/formatter/
