# limine-entry-tool

Un script auxiliar para gestionar las entradas del gestor de arranque Limine en sistemas UEFI.

## Usage
- `limine-entry-tool --scan` : busca otras entradas de arranque UEFI activas y las añade al menú de Limine
- `limine-entry-tool --add "nombre_kernel" "ruta/a/initramfs" "ruta/a/vmlinuz"` : añade una nueva entrada de arranque del kernel con un initramfs y un archivo de kernel
- `limine-entry-tool --add-uki "nombre_kernel" "ruta/a/uki.efi"` : añade una nueva entrada de arranque de imagen de kernel unificada (UKI)
- `limine-entry-tool --remove "nombre_kernel"` : elimina una entrada de arranque del kernel y sus archivos asociados del ESP
- `limine-entry-tool --remove-os "os_name|machine_id"` : elimina una entrada completa del sistema operativo por su nombre o ID de máquina
- `limine-entry-tool --add-efi "nombre_arranque_efi" "ruta/a/loader.efi"` : añade una entrada de arranque EFI para un gestor de arranque alternativo (por ejemplo, Windows)

## Links
- More information: https://gitlab.com/Zesko/limine-entry-tool
