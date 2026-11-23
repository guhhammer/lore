# git format-patch

Prepara archivos `.patch`. Es útil cuando se envían commits por correo electrónico.

## Usage
- `git format-patch origen` : crea un archivo `.patch` con nombre automático para todos los cambios que no están en el push
- `git format-patch revisión_1..revisión_2` : escribe un archivo `.patch` para todos los commits entre dos revisiones a `stdout`
- `git format-patch -3` : escribe un archivo `.patch` para los 3 últimos commits

## Links
- More information: https://git-scm.com/docs/git-format-patch
