# fuzzel

Un lanzador de aplicaciones y buscador difuso nativo de Wayland, inspirado en `rofi` y `dmenu`.

## Usage
- `fuzzel` : ejecuta aplicaciones
- `fuzzel -d` : ejecuta `fuzzel` en modo dmenu
- `ls | fuzzel -d` : muestra un menú con la salida del comando `ls`
- `echo -e "rojo\nverde\nazul" | fuzzel -d` : muestra un menú con elementos personalizados separados por una nueva línea (`\n`)
- `echo -e "rojo\nverde\nazul" | fuzzel -d >color.txt` : permite al usuario elegir entre varios elementos y guarda el seleccionado en un archivo
- `rm -v $HOME/.cache/fuzzel` : restablece el recuento de aplicaciones utilizadas (directorio de caché por defecto: `$XDG_CACHE_HOME/fuzzel`)
- `fuzzel -o "DP-1"` : inicia `fuzzel` en un monitor específico, vea `wlr-randr` o `swaymsg -t get_outputs`
- `fuzzel -d -l 0 --placeholder "Escriba su búsqueda" | sed 's/^/\"/g;s/$/\"/g' | xargs firefox --search` : utilice `fuzzel` para realizar una búsqueda en línea

## Links
- More information: https://codeberg.org/dnkl/fuzzel
