# bash

Bourne-Again SHell, un intérprete de línea de comandos compatible con `sh`.

## Usage
- `bash` : inicia un intérprete de comandos interactivo
- `bash --norc` : inicia el intérprete sin leer archivos de configuración
- `bash -c "comando"` : ejecuta un comando
- `bash archivo.sh` : ejecuta comandos desde un archivo
- `bash -x archivo.sh` : ejecuta comandos desde un archivo, mostrando todos los comando ejecutados en la terminal
- `bash -e archivo.sh` : ejecuta comandos desde un archivo, deteniéndose en el primer error
- `echo "echo 'bash es ejecutado'" | bash` : ejecuta comandos desde `stdin` (entrada estándar)
- `bash -r` : inicia el intérprete restringido

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#Invoking-Bash
