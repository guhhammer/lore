# azcopy

Copia datos desde y hacia Azure Storage.

## Usage
- `azcopy login` : inicia sesión en un inquilino de Azure
- `azcopy c 'ruta/al/archivo_origen' 'https://nombre_cuenta_almacenamiento.blob.core.windows.net/nombre_contenedor/nombre_blob'` : carga un archivo local
- `azcopy c 'ruta/al/directorio_origen' 'https://nombre_cuenta_almacenamiento.blob.core.windows.net/nombre_contenedor' --include-pattern '*.txt;*.jpg'` : carga archivos con extensiones `.txt` y `.jpg`
- `azcopy c 'https://nombre_cuenta_almacenamiento_origen.blob.core.windows.net/nombre_contenedor' 'https://nombre_cuenta_almacenamiento_destino.blob.core.windows.net/nombre_contenedor'` : copia un contenedor directamente entre dos cuentas de almacenamiento de Azure
- `azcopy s 'ruta/al/directorio_origen' 'https://nombre_cuenta_almacenamiento.blob.core.windows.net/nombre_contenedor' --delete-destination true` : sincroniza un directorio local y elimina archivos en el destino si ya no existen en el origen
- `azcopy -h` : muestra la ayuda
