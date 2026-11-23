# semodule

Gestiona los módulos de políticas de SELinux.

## Usage
- `sudo semodule -l` : muestra todos los módulos de políticas instalados
- `sudo semodule -i ruta/al/módulo.pp` : instala un nuevo módulo de políticas
- `sudo semodule -r nombre_del_módulo` : elimina un módulo de políticas
- `sudo semodule -e nombre_del_módulo` : habilita un módulo de políticas
- `sudo semodule -d nombre_del_módulo` : deshabilita un módulo de políticas
- `sudo semodule -R` : recarga todos los módulos de políticas
- `sudo semodule -l -v` : muestra la versión de los módulos de políticas instalados

## Links
- More information: https://manned.org/semodule
