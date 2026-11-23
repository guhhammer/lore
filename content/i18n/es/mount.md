# mount

Accede a un sistema de archivos completo desde un directorio.

## Usage
- `mount` : muestra todos los sistemas de archivos montados
- `mount ruta/al/archivo_de_dispositivo ruta/al/directorio_destino` : monta un dispositivo en un directorio
- `mount -m ruta/al/archivo_de_dispositivo ruta/al/directorio_destino` : crea un directorio específico si no existe y monta un dispositivo en él
- `mount -o uid=id_de_usuario,gid=id_de_grupo ruta/al/archivo_de_dispositivo ruta/al/directorio_destino` : monta un dispositivo en un directorio para un usuario específico
- `mount -t iso9660 -o ro /dev/cdrom /cdrom` : monta un CD-ROM (con el sistema de archivos ISO9660) en `/cdrom` (solo lectura)
- `mount -a` : monta todos los sistemas de archivos definidos en `/etc/fstab`
- `mount ruta/al/punto_de_montaje` : monta un sistema de archivos específico descrito en `/etc/fstab` (por ejemplo, `/dev/sda1 /ruta/al/punto_de_montaje ext2 defaults 0 2`)
- `mount -B ruta/al/directorio_antiguo ruta/al/directorio_nuevo` : monta un directorio en otro directorio

## Links
- More information: https://manned.org/mount.8
