# xclip

Herramienta para manipular el portapapeles de X11, similar a `xsel`. Maneja la selección primaria y secundaria de X y el portapapeles (`<Ctrl c>`/`<Ctrl v>`).

## Usage
- `echo 123 | xclip` : copia la salida de un comando a la selección primaria de X11
- `echo 123 | xclip -se primary|secondary|clipboard` : copia la salida de un commando a una selección de X11
- `echo 123 | xclip -se c` : copia la salida de un commando al portapapeles, usando notación corta
- `xclip -se c archivo.txt` : copia el contenido de un fichero al portapapeles
- `xclip -se c -t image/png archivo.png` : copia el contenido de un fichero con formato PNG al portapapeles
- `xclip -i` : copia la entrada del usuario al portapapeles
- `xclip -o` : imprime el contenido de la selección primaria de X11
- `xclip -o -se c` : imprime el contenido del portapapeles

## Links
- More information: https://manned.org/xclip
