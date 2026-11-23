# mitmdump

Visualiza, registra y transforma mediante programación el tráfico HTTP. La versión de línea de comandos de mitmproxy.

## Usage
- `mitmdump -w ruta/al/archivo` : inicia un proxy y guarda toda la salida en un archivo
- `mitmdump -nr nombre_del_archivo_de_entrada -w nombre_del_archivo_de_salida "~m post"` : filtra un archivo de tráfico guardado para que solo contenga solicitudes POST
- `mitmdump -nc ruta/al/archivo` : reproduce un archivo de tráfico guardado
- `sudo mitmdump -m dns` : intercepta tráfico DNS (inicia un servidor DNS de interceptación en 127.0.0.1:53)

## Links
- More information: https://docs.mitmproxy.org/stable/#mitmdump
