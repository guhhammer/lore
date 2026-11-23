# schroot

Ejecuta un comando o inicia una interfaz interactiva de comando con un directorio raíz diferente. Más personalizable que `chroot`.

## Usage
- `schroot --list` : lista chroots disponibles
- `schroot --chroot chroot comando` : ejecuta un comando en un chroot específico
- `schroot --chroot chroot comando -- opciones_de_comando` : ejecuta un comando con opciones en un chroot específico
- `schroot --all comando` : ejecuta un comando en todos los chroots disponibles
- `schroot --chroot chroot --user usuario` : inicia una shell interactiva dentro de un chroot específico como usuario específico
- `schroot --begin-session --chroot chroot` : inicia una nueva sesión (devuelve un identificador único en `stdout`)
- `schroot --run-session --chroot id_de_sesión` : se conecta a una sesión existente
- `schroot --end-session --chroot id_de_sesión` : finaliza una sesión en curso

## Links
- More information: https://wiki.debian.org/Schroot
