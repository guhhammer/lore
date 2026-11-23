# aa-audit

Establece perfiles de seguridad de AppArmor en modo de auditoría.

## Usage
- `sudo aa-audit nombre_perfil` : establece un perfil en modo auditoría
- `sudo aa-audit perfil1 perfil2 ...` : establece múltiples perfiles en modo auditoría
- `sudo aa-audit -d /ruta/a/perfiles nombre_perfil` : establece un perfil en modo auditoría desde un directorío específico
- `sudo aa-audit --force nombre_perfil` : fuerza modo auditoría incluso si ya ha sido aplicado
- `sudo aa-audit --no-reload nombre_perfil` : establece un perfil en modo auditoría sin recargarlo
- `sudo aa-audit -r nombre_perfil` : elimina el modo auditoría de un perfil
- `aa-audit -h` : muestra la ayuda

## Links
- More information: https://gitlab.com/apparmor/apparmor/-/wikis/manpage_aa-audit.8
