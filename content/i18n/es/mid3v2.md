# mid3v2

Edita etiquetas de audio.

## Usage
- `mid3v2 --list-frames ruta/al/archivo1.mp3 ruta/al/archivo2.mp3 ...` : lista de todos los marcos ID3v2.3 o ID3v2.4 admitidos y sus significados
- `mid3v2 --list-genres ruta/al/archivo1.mp3 ruta/al/archivo2.mp3 ...` : lista de todos los géneros numéricos ID3v1 admitidos
- `mid3v2 --list ruta/al/archivo1.mp3 ruta/al/archivo2.mp3 ...` : lista todas las etiquetas en archivos específicos
- `mid3v2 --artist|--album|--song=string ruta/al/archivo1.mp3 ruta/al/archivo2.mp3 ...` : establece información específica sobre artistas, álbumes o canciones
- `mid3v2 --picture=filename:description:image_type:mime_type ruta/al/archivo1.mp3 ruta/al/archivo2.mp3 ...` : establece información específica de la imagen
- `mid3v2 --year=YYYY ruta/al/archivo1.mp3 ruta/al/archivo2.mp3 ...` : establece información específica del año
- `mid3v2 --date=YYYY-MM-DD ruta/al/archivo1.mp3 ruta/al/archivo2.mp3 ...` : establece información de fecha específica

## Links
- More information: https://mutagen.readthedocs.io/en/latest/man/mid3v2.html
