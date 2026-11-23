# choco source

Gestiona fuentes para paquetes con Chocolatey.

## Usage
- `choco source list` : lista las fuentes actualmente disponibles
- `choco source add -n nombre -s url` : agrega una nueva fuente de paquete
- `choco source add -n nombre -s url -u usuario -p contraseña` : agrega una nueva fuente de paquete con credenciales
- `choco source add -n nombre -s url --cert ruta\al\archivo_certificado` : agrega una nueva fuente de paquete con un certificado de cliente
- `choco source enable -n nombre` : habilita una fuente de paquete
- `choco source disable -n nombre` : deshabilita una fuente de paquete
- `choco source remove -n nombre` : elimina una fuente de paquete

## Links
- More information: https://docs.chocolatey.org/en-us/choco/commands/source/
