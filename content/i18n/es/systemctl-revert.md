# systemctl revert

Restaura los archivos de unidad a sus versiones de origen. Deshace los efectos de `edit`, `enable`, `disable`, `set-property` y `mask`.

## Usage
- `systemctl revert unidad1 unidad2 ...` : restaura los archivos de unidad a su configuración predeterminada
- `systemctl revert unidad --user` : restaura un archivo de unidad de usuario
