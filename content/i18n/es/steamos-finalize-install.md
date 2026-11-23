# steamos-finalize-install

Finaliza la instalación de SteamOS configurando los cargadores de arranque y aplicando las actualizaciones del sistema.

## Usage
- `sudo steamos-finalize-install` : finaliza la instalación
- `sudo steamos-finalize-install --no-bootloaders --no-kernel` : finaliza sin actualizar los cargadores de arranque ni el núcleo
- `sudo steamos-finalize-install --no-migrate` : omite todos los pasos de migración
- `sudo steamos-finalize-install --roothash hash` : establece un hash de raíz específico durante la finalización
- `sudo steamos-finalize-install --force` : fuerza los pasos de migración del sistema independientemente del entorno

## Links
- More information: https://gitlab.com/users/evlaV/projects
