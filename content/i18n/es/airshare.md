# airshare

Transfiere datos entre dos máquinas en una red local.

## Usage
- `airshare code ruta/al/archivo_o_directorio1 ruta/al/archivo_o_directorio2 ...` : comparte archivos o directorios
- `airshare code` : recibe un archivo
- `airshare --upload code` : aloja un servidor receptor (usa esto para poder cargar archivos usando la interfaz web)
- `airshare --upload code ruta/al/archivo_o_directorio1 ruta/al/archivo_o_directorio2 ...` : envía archivos o directorios a un servidor receptor
- `airshare --file-path code` : envía archivos cuyas rutas han sido copiadas al portapapeles
- `airshare --clip-receive code` : recibe un archivo y lo copia al portapapeles

## Links
- More information: https://airshare.readthedocs.io/en/latest/cli.html
