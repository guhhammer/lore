# grubby

Herramienta para configurar los gestores de arranque `grub` y `zipl`.

## Usage
- `sudo grubby --update-kernel=ALL --args 'quiet console=ttyS0'` : añade argumentos de arranque del núcleo a todas las entradas del menú del núcleo
- `sudo grubby --update-kernel=DEFAULT --remove-args quiet` : elimina los argumentos existentes de la entrada para el núcleo por defecto
- `sudo grubby --info=ALL` : lista todas las entradas del menú del núcleo

## Links
- More information: https://manned.org/grubby.8
