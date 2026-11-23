# bspc

Configura y controla `bspwm`, gestionando nodos, escritorios, monitores y más.

## Usage
- `bspc monitor -d nombre_escritorio1 nombre_escritorio2` : define dos escritorios virtuales
- `bspc desktop -f número` : enfoca un determinado escritorio
- `bspc node -c` : cierra las ventanas enraizadas en el nodo seleccionado
- `bspc node -d número` : envía el nodo seleccionado al escritorio especificado
- `bspc node -t ~fullscreen` : activa el modo de pantalla completa para el nodo seleccionado
- `bspc config nombre_configuración valor` : establece el valor de una configuración determinada

## Links
- More information: https://github.com/baskerville/bspwm/blob/master/doc/bspwm.1.asciidoc
