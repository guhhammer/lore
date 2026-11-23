# ausearch

Consulta el registro de auditoría de Linux en busca de eventos. Parte del paquete `audit`.

## Usage
- `sudo ausearch -m avc` : busca todos los eventos de denegación AVC de SELinux
- `sudo ausearch -c httpd` : busca eventos relacionados con un ejecutable específico
- `sudo ausearch -ui 1000` : busca eventos de un usuario específico
- `sudo ausearch -ts recent` : busca eventos en los últimos 10 minutos
- `sudo ausearch -m user_login -sv no` : busca intentos de inicio de sesión fallidos
- `sudo ausearch -f ruta/al/archivo` : busca eventos relacionados con un archivo específico
- `sudo ausearch -m avc --raw` : muestra los resultados en formato sin procesar para su posterior procesamiento

## Links
- More information: https://manned.org/ausearch
