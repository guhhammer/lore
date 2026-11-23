# ruff check

Un linter extremadamente rápido para Python. `check` es el comando predeterminado - se puede omitir en todas partes. Si no se especifican archivos o directorios, el directorio de trabajo actual se utiliza por defecto.

## Usage
- `ruff check ruta/al/archivo_o_directorio1 ruta/al/archivo_o_directorio2 ...` : ejecuta el linter en los archivos o directorios dados
- `ruff check --fix` : aplica las correcciones sugeridas, modificando los archivos afectados
- `ruff check --watch` : ejecuta el linter y lo aplica de nuevo ante algún cambio
- `ruff check --select ALL|regla_de_código1,regla_de_código2,...` : solo habilita las reglas especificadas (o todas las reglas), ignorando el archivo de configuración
- `ruff check --extend-select regla_de_código1,regla_de_código2,...` : además, habilita las reglas especificadas
- `ruff check --ignore regla_de_código1,regla_de_código2,...` : desactiva las reglas especificadas
- `ruff check --select regla_de_código --add-noqa` : ignora todas las violaciones existentes de una regla añadiendo las directivas "# noqa" a todas las líneas que la violan

## Links
- More information: https://docs.astral.sh/ruff/linter/
