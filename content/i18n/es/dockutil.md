# dockutil

Gestiona los elementos del dock de macOS.

## Usage
- `dockutil --add ruta/a/la/aplicación` : añade una aplicación al final del dock del usuario actual
- `dockutil --add /ruta/a/la/aplicación --replacing 'etiqueta_de_elemento_del_dock'` : reemplaza una aplicación por otra en el dock del usuario actual
- `dockutil --add /ruta/al/directorio --view grid|fan|list|auto --display folder|stack` : añade un directorio con opciones de visualización y lo muestra como un icono de carpeta o pila
- `dockutil --add vnc://ejemplo_servidor.local --label 'VNC de ejemplo' --after etiqueta_de_elemento_del_dock` : añade la URL de un elemento del dock después de otro elemento
- `dockutil --remove 'etiqueta_de_elemento_del_dock'` : elimina una aplicación del dock dado su nombre de etiqueta en el dock
- `dockutil --add '' --type spacer|small-spacer|flex-spacer --section apps --after etiqueta_de_elemento_del_dock` : añade un espaciador en una sección después de una aplicación
- `dockutil --remove spacer-tiles` : elimina todos los elementos espaciadores

## Links
- More information: https://github.com/kcrawford/dockutil
