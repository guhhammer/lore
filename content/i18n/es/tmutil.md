# tmutil

Utilidad para gestionar las copias de seguridad de Time Machine. La mayoría de las acciones requieren privilegios de root.

## Usage
- `sudo tmutil setdestination ruta/al/punto_montaje_disco` : establece una unidad HFS+ como destino de la copia de seguridad
- `sudo tmutil setdestination "protocolo://usuario:contraseña@host/compartir"` : establece un recurso compartido APF o SMB como destino de la copia de seguridad
- `sudo tmutil setdestination -a destino` : añade el destino indicado a la lista de destinos
- `sudo tmutil enable` : activa copias de seguridad automáticas
- `sudo tmutil disable` : desactiva las copias de seguridad automáticas
- `sudo tmutil startbackup` : inicia una copia de seguridad, si ya no se está ejecutando, y libera el control del intérprete de comandos
- `sudo tmutil startbackup -b` : inicia una copia de seguridad y bloquearla hasta que termine
- `sudo tmutil stopbackup` : detiene una copia de seguridad

## Links
- More information: https://keith.github.io/xcode-man-pages/tmutil.8.html
