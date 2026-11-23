# immich-go

Immich-Go es una herramienta abierta diseñada para subir grandes colecciones de fotos a tu servidor Immich autoalojado.

## Usage
- `immich-go -server=url_del_servidor -key=clave_de_servidor upload ruta/al/archivo_takeout.zip` : sube un archivo takeout de Google al servidor Immich
- `immich-go -server=url_del_servidor -key=clave_del_servidor upload -create-albums -google-photos -date=2019-06 ruta/al/archivo_takeout.zip` : importa fotos capturadas en junio del 2019, mientras se generan los álbumes automáticamente
- `immich-go -use-configuration=~/.immich-go/immich-go.json upload ruta/al/archivo_takeout.zip` : sube un archivo usando servidor y clave de un archivo de configuración
- `immich-go -server=url_del_servidor -key=clave_del_servidor duplicate -yes` : examina el contenido del servidor Immich, elimina las imágenes de menor calidad y preserva álbumes
- `immich-go -server=url_del_servidor -key=clave_del_servidor tool album delete \d{4}-\d{2}-\d{2}` : elimina todos los álbumes creados con el patrón "YYYY-MM-DD"

## Links
- More information: https://github.com/simulot/immich-go
