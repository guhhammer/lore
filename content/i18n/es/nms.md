# nms

Herramienta de línea de comandos que recrea el famoso efecto de desencriptado de datos de la película Sneakers (1992).

## Usage
- `echo "Hola, Mundo!" | nms` : desencripta el texto tras presionar una tecla
- `ls -la | nms -a` : desencripta la salida inmediatamente, sin esperar a que una tecla sea pulsada
- `cat ruta/al/archivo | nms -a -f blue|white|yellow|black|magenta|green|red` : desencripta el contenido de un archivo, especificando el color de la salida
- `comando | nms -a -c` : limpia la pantalla antes de desencriptar

## Links
- More information: https://github.com/bartobri/no-more-secrets
