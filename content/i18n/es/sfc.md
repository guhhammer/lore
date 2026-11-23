# sfc

Escanea la integridad de los archivos del sistema de Windows.

## Usage
- `sfc /scannow` : escanea todos los archivos del sistema y, si es posible, reparar cualquier problema
- `sfc /verifyonly` : escanea todos los archivos del sistema sin intentar reparar ninguno
- `sfc /scanfile=ruta\al\archivo` : escanea un archivo específico y, si es posible, reparar cualquier problema
- `sfc /verifyfile=ruta\al\archivo` : escanea un archivo específico sin intentar repararlo
- `sfc /offbootdir=ruta\al\directorio` : al reparar sin conexión, especificar el directorio de arranque
- `sfc /offwindir=ruta\al\directorio` : al reparar sin conexión, especificar el directorio de Windows
- `sfc` : muestra la ayuda
