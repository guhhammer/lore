# xsel

Herramienta X11 de selección y manipulación del portapapeles.

## Usage
- `echo 123 | xsel -ib` : utiliza la salida de un comando como entrada del portapapeles (clipboard) (equivalente a `<Ctrl c>`)
- `cat ruta/al/archivo | xsel -ib` : utiliza el contenido de un archivo como entrada del portapapeles
- `xsel -ob` : envía el contenido del portapapeles a la terminal (equivalente a `<Ctrl v>`)
- `xsel -ob >ruta/al/archivo` : envía el contenido del portapapeles a un archivo
- `xsel -cb` : limpia el portapapeles
- `xsel -op` : envía el contenido de la selección primaria X11 a la terminal (equivalente a `<MiddleClick>`)

## Links
- More information: https://manned.org/xsel
