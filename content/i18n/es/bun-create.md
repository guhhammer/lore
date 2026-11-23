# bun create

Crea un nuevo proyecto a partir de una plantilla.

## Usage
- `bun c plantilla` : crea un nuevo proyecto a partir de una plantilla oficial de forma interactiva
- `bun c plantilla ruta/al/destino` : crea un nuevo proyecto a partir de una plantilla oficial en un nuevo directorio
- `bun c https://github.com/username/repo ruta/al/destino` : crea un nuevo proyecto a partir de una plantilla de un repositorio de GitHub
- `bun c ruta/a/plantilla ruta/al/destino` : crea un nuevo proyecto a partir de una plantilla local
- `bun c plantilla ruta/al/destino --force` : crea un nuevo proyecto, sobrescribiendo el directorio de destino si ya existe
- `bun c plantilla ruta/al/destino --no-git` : crea un nuevo proyecto sin inicializar automáticamente un repositorio de Git
- `bun c plantilla ruta/al/destino --no-install` : crea un nuevo proyecto sin instalar automáticamente las dependencias

## Links
- More information: https://bun.com/docs/runtime/templating/create
