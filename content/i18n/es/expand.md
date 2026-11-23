# expand

Descomprime archivos Cabinet de Windows.

## Usage
- `expand ruta\al\archivo.cab ruta\al\directorio` : descomprime un archivo Cabinet de un solo archivo en el directorio especificado
- `expand ruta\al\archivo.cab ruta\al\directorio -d` : muestra la lista de archivos en un archivo Cabinet de origen
- `expand ruta\al\archivo.cab ruta\al\directorio -f:*` : descomprime todos los archivos del archivo Cabinet
- `expand ruta\al\archivo.cab ruta\al\directorio -f:ruta\al\archivo` : descomprime un archivo específico de un archivo Cabinet
- `expand ruta\al\archivo.cab ruta\al\directorio -i` : ignora la estructura de directorios al descomprimir y los agregar a un solo directorio
