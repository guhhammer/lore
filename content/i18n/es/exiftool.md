# exiftool

Lee y escribe metainformación en archivos.

## Usage
- `exiftool ruta/al/archivo` : muestra los metadatos EXIF de un archivo dado
- `exiftool -All= ruta/al/archivo1 ruta/al/archivo2 ...` : elimina todos los metadatos EXIF de los archivos dados
- `exiftool "-gps*=" ruta/a/imagen1 ruta/a/imagen2 ...` : elimina los metadatos EXIF de GPS de los archivos de imagen dados
- `exiftool -All= -tagsfromfile @ -colorspacetags -orientation ruta/a/imagen1 ruta/a/imagen2 ...` : elimina todos los metadatos EXIF de los archivos de imagen dados y luego vuelve a añadir los metadatos de color y orientación
- `exiftool "-AllDates+=0:0:0 1:0:0" ruta/al/directorio` : adelanta 1 hora la fecha en que se tomaron todas las fotos de un directorio
- `exiftool "-AllDates-=0:0:1 2:0:0" -ext jpg` : retrasa 1 día y 2 horas la fecha en que se tomaron todas las fotos JPEG del directorio actual
- `exiftool -DateTimeOriginal-=1.5 -overwrite_original` : modifica únicamente el campo `DateTimeOriginal` restando 1,5 horas, sin mantener copias de seguridad
- `exiftool '-filename<DateTimeOriginal' -d %Y-%m-%d_%H-%M-%S%%lc.%%e ruta/al/directorio -r -ext jpg` : renombra de forma recursiva todas las fotos JPEG de un directorio según el campo `DateTimeOriginal`

## Links
- More information: https://exiftool.org/exiftool_pod.html
