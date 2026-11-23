# git cliff

Un generador de registros de cambios altamente personalizable.

## Usage
- `git cliff >CHANGELOG.md` : genera un registro de cambios a partir de todos los commits de un repositorio Git y lo guarda en `CHANGELOG.md`
- `git cliff -l` : genera un registro de cambios a partir de las confirmaciones desde la última etiqueta y lo imprime en `stdout`
- `git cliff --current` : genera un registro de cambios a partir de las confirmaciones que pertenecen a la etiqueta actual (usa `git checkout` en una etiqueta anterior a esta)
- `git cliff -u` : genera un registro de cambios a partir de las confirmaciones que no pertenecen a una etiqueta
- `git cliff -i` : escribe el archivo de configuración por defecto en `cliff.toml` en el directorio actual

## Links
- More information: https://git-cliff.org/docs/usage/args/
