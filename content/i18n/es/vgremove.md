# vgremove

Elimina grupo(s) de volúmenes en LVM.

## Usage
- `sudo vgremove grupo_volumen` : elimina un grupo de volúmenes con confirmación
- `sudo vgremove -f grupo_volumen` : fuerza la eliminación de un grupo de volúmenes sin confirmación
- `sudo vgremove -d -d grupo_volumen` : establece el nivel de depuración para el registro detallado en el nivel 2, (repite `--debug` hasta 6 veces para aumentar el nivel)
- `sudo vgremove --config 'global/locking_type=1' grupo_volumen` : utiliza una configuración específica para anular los valores predeterminados
- `vgremove -h` : muestra la ayuda

## Links
- More information: https://manned.org/vgremove
