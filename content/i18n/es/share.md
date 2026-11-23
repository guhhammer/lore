# share

Hace que el recurso/sistema de archivos local esté disponible para ser montado por sistemas remotos.

## Usage
- `share` : lista todos los sistemas de archivos compartidos actualmente
- `share -F nfs -o rw /ruta/al/directorio` : comparte un directorio con acceso de lectura/escritura
- `share -F nfs -o ro /ruta/al/directorio` : comparte un directorio con acceso de solo lectura
- `share -F nfs -o rw,root=nombre_del_host /ruta/al/directorio` : comparte un directorio con opciones específicas (por ejemplo, permitir el acceso root desde un host concreto)
- `echo "share -F nfs -o rw /ruta/al/directorio" >>/etc/dfs/dfstab` : hace que la compartición sea persistente añadiendo entradas a `/etc/dfs/dfstab`

## Links
- More information: https://docs.oracle.com/cd/E36784_01/html/E36825/gntjt.html
