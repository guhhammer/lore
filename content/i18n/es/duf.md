# duf

Utilidad de uso/espacio libre de disco.

## Usage
- `duf` : lista los dispositivos accesibles
- `duf --all` : lista todo (como sistemas de archivos pseudo, duplicados o inaccesibles)
- `duf ruta/al/directorio1 ruta/al/directorio2 ...` : muestra solo los dispositivos o puntos de montaje especificados
- `duf --sort size|used|avail|usage` : ordena la salida según un criterio específico
- `duf --only-fs|hide-fs tmpfs|vfat|ext4|xfs` : muestra u oculta sistemas de archivos específicos
- `duf --sort mountpoint|size|used|avail|usage|inodes|inodes_used|inodes_avail|inodes_usage|type|filesystem` : ordena la salida por clave
- `duf --theme dark|light` : cambia el tema (si `duf` no consigue usar el tema correcto)

## Links
- More information: https://github.com/muesli/duf#usage
