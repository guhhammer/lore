# git clone

Clona un repositorio existente.

## Usage
- `git clone ubicación_repositorio_remoto ruta/al/directorio` : clona un repositorio existente en un directorio nuevo (el directorio por defecto es el nombre del repositorio)
- `git clone --recursive ubicación_repositorio_remoto` : clona un repositorio existente y sus submódulos
- `git clone -n ubicación_repositorio_remoto` : clona sólo el directorio `.git` de un repositorio existente
- `git clone -l ruta/al/repositorio/local` : clona un repositorio local
- `git clone -q ubicación_repositorio_remoto` : clona en silencio
- `git clone --depth 10 ubicación_repositorio_remoto` : clona un repositorio existente obteniendo sólo los 10 commits más recientes de la rama por defecto (útil para ahorrar tiempo)
- `git clone -b nombre --single-branch ubicación_repositorio_remoto` : clona un repositorio existente buscando sólo una rama específica
- `git clone -c core.sshCommand="ssh -i ruta/a/clave_privada_ssh" ubicación_repositorio_remoto` : clona un repositorio existente usando un comando SSH específico

## Links
- More information: https://git-scm.com/docs/git-clone
