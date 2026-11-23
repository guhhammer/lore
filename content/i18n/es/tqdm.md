# tqdm

Muestra una barra de progreso a lo largo del tiempo de un comando.

## Usage
- `seq 10000000 | tqdm | comando` : muestra iteraciones por segundo y usa `stdout` después
- `seq 10000000 | tqdm --total 10000000 | comando` : crea una barra de progreso
- `zip -r ruta/al/archivo.zip ruta/al/directorio | tqdm --total $(find ruta/al/directorio | wc -l) --unit files --null` : crea un archivo a partir de un directorio y utiliza el recuento de archivos de ese directorio para crear una barra de progreso
- `tar vzcf ruta/al/archivo.tar.gz ruta/al/directorio 2>&1 | tqdm --total $(find ruta/al/directorio | wc -l) --unit files --null` : crea un archivo con tar y crea una barra de progreso (sistema agnóstico, GNU tar usa `stdout` mientras BSD tar usa `stderr`)

## Links
- More information: https://tqdm.github.io/docs/cli/
