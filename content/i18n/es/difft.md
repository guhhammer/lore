# difft

Compara archivos o directorios en base de la sintaxis del lenguaje de programación.

## Usage
- `difft ruta/al/archivo_o_directorio1 ruta/al/archivo_o_directorio2` : compara dos archivos o directorios
- `difft --check-only ruta/al/archivo1 ruta/al/archivo2` : informa únicamente diferencias entre los archivos
- `difft --display side-by-side|side-by-side-show-both|inline|json ruta/al/archivo1 ruta/al/archivo2` : especifica el modo de visualización (por defecto es `side-by-side`)
- `difft --ignore-comments ruta/al/archivo1 ruta/al/archivo2` : ignora comentarios al comparar
- `difft --syntax-highlight on|off ruta/al/archivo1 ruta/al/archivo2` : activa o desactiva el resaltado sintáctico del código fuente (por defecto está activado)
- `difft --skip-unchanged ruta/al/archivo_o_directorio1 ruta/al/archivo_o_directorio2` : silenciosamente omite los archivos que no hayan cambiado
- `difft --list-languages` : lista todos los lenguajes de programación soportados por la herramienta, junto con sus extensiones

## Links
- More information: https://difftastic.wilfred.me.uk/introduction.html
