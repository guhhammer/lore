# rdpsign

Una herramienta para firmar archivos del Protocolo de Escritorio Remoto (RDP).

## Usage
- `rdpsign ruta\al\archivo.rdp` : firma un archivo RDP
- `rdpsign ruta\al\archivo.rdp /sha265 hash` : firma un archivo RDP utilizando un hash sha256 específico
- `rdpsign ruta\al\archivo.rdp /q` : habilita salida silenciosa
- `rdpsign ruta\al\archivo.rdp /v` : muestra advertencias, mensajes y estados detallados
- `rdpsign ruta\al\archivo.rdp /l` : prueba la firma mostrando la salida en `stdout` sin actualizar el archivo
