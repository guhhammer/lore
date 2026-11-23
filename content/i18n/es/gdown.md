# gdown

Descarga archivos desde Google Drive y otras URLs.

## Usage
- `gdown url` : descarga un archivo desde una URL
- `gdown id_de_archivo` : descarga usando un ID de archivo
- `gdown --fuzzy url` : descarga con extracción de ID de archivo difuso (también funciona con enlaces <https://docs.google.com>)
- `gdown id_de_carpeta|url -O ruta/al/directorio_de_salida --folder` : descarga una carpeta utilizando su ID o la URL completa
- `gdown tar_url -O - -q | tar xvf -` : descarga un archivo `.tar`, escríbelo en `stdout` y extráelo

## Links
- More information: https://github.com/wkentaro/gdown#usage
