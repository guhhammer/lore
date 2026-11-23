# fsutil

Muestra información sobre volúmenes del sistema de archivos.

## Usage
- `fsutil volume list` : muestra una lista de volúmenes
- `fsutil fsInfo volumeInfo letra_de_unidad|ruta_del_volumen` : muestra información sobre el sistema de archivos de un volumen
- `fsutil repair state` : muestra el estado actual de la reparación automática del sistema de archivos para todos los volúmenes
- `fsutil dirty query` : muestra el estado del bit sucio de todos los volúmenes
- `fsutil dirty set letra_de_unidad|ruta_del_volumen` : establece el estado del bit sucio de un volumen
