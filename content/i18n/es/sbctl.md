# sbctl

Un gestor de claves de arranque seguro fácil de usar.

## Usage
- `sbctl status` : muestra el estado actual del arranque seguro
- `sbctl create-keys` : crea claves de arranque seguro personalizadas (todo se almacena en `/var/lib/sbctl` de forma predeterminada)
- `sbctl enroll-keys -m` : inscribe las claves de arranque seguro personalizadas y los certificados de proveedor UEFI de Microsoft
- `sbctl setup --setup` : ejecuta automáticamente `create-keys` y `enroll-keys` basado en los parámetros de `/etc/sbctl/sbctl.conf`
- `sbctl sign -s ruta/al/binario_efi` : firma un binario EFI con la clave creada y guarda el archivo en la base de datos
- `sbctl sign-all` : vuelve a firmar todos los archivos guardados
- `sbctl verify` : comprueba que se han firmado todos los ejecutables EFI de la partición EFI del sistema

## Links
- More information: https://github.com/Foxboron/sbctl/wiki/FAQ#option-rom
