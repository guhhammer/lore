# clamscan

Un escáner de virus de línea de comandos.

## Usage
- `clamscan ruta/al/archivo` : escanea un archivo buscando vulnerabilidades
- `clamscan -r ruta/al/directorio` : escanea todos los archivos recursivamente en un directorio específico
- `comando | clamscan -` : escanea datos desde `stdin`
- `clamscan -d ruta/al/archivo_o_directorio_con_base_de_datos` : escanea usando un archivo de bases de datos de virus o directorio de archivos
- `clamscan -i` : escanea el directorio actual y muestra solo los archivos infectados
- `clamscan -l ruta/al/archivo_de_registro` : imprime el informe de la exploración a un archivo de registro (log)
- `clamscan --move ruta/al/directorio_de_cuarentena` : mueve archivos infectados a un directorio específico
- `clamscan --remove yes` : elimina los archivos infectados

## Links
- More information: https://docs.clamav.net/manual/Usage/Scanning.html#clamscan
