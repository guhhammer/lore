# dmenu

Menú dinámico. Crea un menú a partir de una entrada de texto con cada elemento, en una nueva línea.

## Usage
- `ls | dmenu` : muestra un menú a partir de la salida del comando 'ls'
- `echo -e "rojo\nverde\nazul" | dmenu` : muestra un menú con artículos personalizados separados por una nueva línea (`\n`)
- `echo -e "rojo\nverde\nazul" | dmenu >color.txt` : deja que el usuario elija entre varios elementos y guarda el seleccionado en un archivo
- `ls | dmenu -m 1` : lanza dmenu en un monitor específico
- `ls | dmenu -b` : muestra dmenu en la parte inferior de la pantalla

## Links
- More information: https://manned.org/dmenu
