# fsck

Comprueba la integridad de un sistema de archivos o los repara. El sistema de ficheros debe estar desmontado en el momento de ejecutar el comando. Es una envoltura que llama a `fsck_hfs`, `fsck_apfs`, `fsck_msdos`, `fsck_exfat`, y `fsck_udf` según sea necesario.

## Usage
- `fsck /dev/sdX` : comprueba el sistema de ficheros `/dev/sdX`, informando de cualquier bloque dañado
- `fsck -f /dev/sdX` : comprueba el sistema de ficheros `/dev/sdX` sólo si está limpio, informando de los bloques dañados y dejando que el usuario elija interactivamente si repara cada uno de ellos
- `fsck -fy /dev/sdX` : comprueba el sistema de archivos `/dev/sdX` sólo si está limpio, informando de los bloques dañados y reparándolos automáticamente
- `fsck -q /dev/sdX` : comprueba el sistema de archivos `/dev/sdX`, informando si se ha desmontado correctamente

## Links
- More information: https://keith.github.io/xcode-man-pages/fsck.8.html
