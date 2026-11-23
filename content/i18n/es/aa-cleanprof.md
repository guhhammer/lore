# aa-cleanprof

Limpia perfiles de seguridad de AppArmor eliminando reglas sin utilizar.

## Usage
- `sudo aa-cleanprof nombre_perfil` : limpia un usuario
- `sudo aa-cleanprof perfil1 perfil2 ...` : limpia múltiples perfiles al mismo tiempo
- `sudo aa-cleanprof -d /ruta/a/perfiles nombre_perfil` : limpia un perfil desde un directorío específico
- `sudo aa-cleanprof -s nombre_perfil` : ejecuta silenciosamente sin indicaciones
- `sudo aa-cleanprof --no-reload nombre_perfil` : prevén la recarga del perfil tras limpiarlo
- `aa-cleanprof -h` : muestra la ayuda

## Links
- More information: https://gitlab.com/apparmor/apparmor/-/wikis/manpage_aa-cleanprof.8
