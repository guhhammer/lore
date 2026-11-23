# git blame

Muestra el hash de la confirmación y el último autor en cada línea de un archivo.

## Usage
- `git blame ruta/al/archivo` : imprime el archivo con el nombre del autor y el hash del commit en cada línea
- `git blame -e ruta/al/archivo` : imprime el archivo con el correo electrónico del autor y el hash de confirmación en cada línea
- `git blame commit ruta/al/archivo` : imprime el archivo con el nombre del autor y el hash de confirmación en cada línea en una confirmación específica
- `git blame commit~ ruta/al/archivo` : imprime el archivo con el nombre del autor y el hash de confirmación en cada línea antes de una confirmación específica
- `git blame -L '/text/',+10 a82812aa^ ruta/al/archivo` : salta al origen de una confirmación específica y rastrea un texto específico y 10 de sus líneas siguientes
- `git blame -L start_line,end_line ruta/al/archivo` : imprime el nombre del autor y la información del hash de confirmación para un rango de líneas específico
- `git blame -w -C -C -C ruta/al/archivo` : ignora los espacios en blanco y los movimientos de línea

## Links
- More information: https://git-scm.com/docs/git-blame
