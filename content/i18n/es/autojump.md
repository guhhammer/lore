# autojump

Salta rápidamente entre los directorios que más visitas. Se proporcionan alias como `j` o `jc` para escribir aún menos.

## Usage
- `source /usr/share/autojump/autojump.bash|fish|zsh` : añade los alias `autojump` a tu intérprete de comandos
- `j patrón` : salta a un directorio que contenga el patrón dado
- `jc patrón` : salta a un subdirectorio (hijo) del directorio actual que contenga el patrón dado
- `jo patrón` : abre un directorio que contenga el patrón dado en el gestor de archivos del sistema operativo
- `j --purge` : elimina directorios que no existen de la base de datos `autojump`
- `j -s` : mustra las entradas de la base de datos `autojump`

## Links
- More information: https://github.com/wting/autojump#name
