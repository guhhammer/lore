# clamdscan

Escaneo de virus con el servicio (daemon) ClamAV.

## Usage
- `clamdscan ruta/al/archivo_o_directorio` : escanea un archivo o directorio buscando vulnerabilidades
- `comando | clamdscan -` : escanea desde `stdin`
- `clamdscan --infected` : escanea el directorio actual y muestra solo los archivos infectados
- `clamdscan --log ruta/al/archivo_de_registro` : imprime el informe de la exploración a un archivo de registro (log)
- `clamdscan --move ruta/al/directorio_de_cuarentena` : mueve los archivos infectados a un directorio específico
- `clamdscan --remove` : elimina los archivos infectados
- `clamdscan --multiscan` : utiliza varios hilos para escanear un directorio
- `clamdscan --fdpass` : pasa el descriptor de archivo en lugar de transmitir el archivo al daemon

## Links
- More information: https://docs.clamav.net/manual/Usage/Scanning.html#clamdscan
