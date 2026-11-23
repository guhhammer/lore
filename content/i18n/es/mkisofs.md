# mkisofs

Crea archivos ISO a partir de directorios. También conocido como `genisoimage`.

## Usage
- `mkisofs -o nombre_archivo.iso ruta/al/directorio_de_origen` : crea un ISO a partir de un directorio
- `mkisofs -o nombre_archivo.iso -V "nombre_etiqueta" ruta/al/directorio_de_origen` : establece la etiqueta del disco al crear una ISO
- `mkisofs -o nombre_archivo.iso -allow-limited-size ruta/al/directorio_de_origen` : crea una imagen ISO con archivos de más de 2 GiB indicando un tamaño aparente menor para sistemas de archivos ISO9660

## Links
- More information: https://manned.org/mkisofs
