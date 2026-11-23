# git bundle

Empaqueta objetos y referencias en un archivo.

## Usage
- `git bundle create ruta/al/archivo.bundle nombre_rama` : crea un archivo bundle que contiene todos los objetos y referencias de una rama específica
- `git bundle create ruta/al/archivo.bundle --all` : crea un archivo bundle de todas las ramas
- `git bundle create ruta/al/archivo.bundle -5 HEAD` : crea un archivo bundle de los últimos 5 commits de la rama actual
- `git bundle create ruta/al/archivo.bundle --since 7.days HEAD` : crea un archivo bundle de los últimos 7 días
- `git bundle verify ruta/al/archivo.bundle` : verifica que un archivo bundle es válido y puede aplicarse al repositorio actual
- `git bundle unbundle ruta/al/archivo.bundle` : imprime en `stdout` la lista de referencias contenidas en un bundle
- `git pull ruta/al/archivo.bundle nombre_rama` : desagrupa una rama específica de un archivo bundle en el repositorio actual
- `git clone ruta/al/archivo.bundle` : crea un nuevo repositorio a partir de un paquete

## Links
- More information: https://git-scm.com/docs/git-bundle
