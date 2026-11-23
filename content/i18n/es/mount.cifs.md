# mount.cifs

Monta SMB (Server Message Block) o CIFS (Common Internet File System). Nota: también puede hacer lo mismo pasando la opción `-t cifs` a `mount`.

## Usage
- `mount.cifs -o user=usuario //servidor/nombre_del_share punto_de_montaje` : conecta el nombre de usuario especificado o `$USER` por defecto (se le pedirá una contraseña)
- `mount.cifs -o guest //servidor/nombre_del_share punto_de_montaje` : conecta como usuario invitado (sin contraseña)
- `mount.cifs -o uid=id_del_usuario|usuario,gid=id_del_grupo|nombre_del_grupo //servidor/nombre_del_share punto_de_montaje` : establece información de propiedad para el directorio montado

## Links
- More information: https://manned.org/mount.cifs
