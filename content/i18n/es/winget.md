# winget

Administrador de paquetes de Windows.

## Usage
- `winget add paquete` : instala un paquete
- `winget rm paquete` : elimina un paquete (Nota: `remove` también se puede usar en lugar de `uninstall`)
- `winget show paquete` : muestra información sobre un paquete
- `winget search paquete` : busca un paquete
- `winget upgrade -r` : actualiza todos los paquetes a las versiones más recientes
- `winget ls -s winget` : lista todos los paquetes instalados que se pueden gestionar con `winget`
- `winget import|export --import-file|--output ruta/al/archivo` : importa paquetes desde un archivo o exportar paquetes instalados a un archivo
- `winget validate ruta/al/manifiesto` : valida manifiestos antes de enviar un PR al repositorio winget-pkgs

## Links
- More information: https://learn.microsoft.com/windows/package-manager/winget
