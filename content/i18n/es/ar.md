# ar

Crea, modifica y extrae de archivos Unix. Típicamente utilizado para bibliotecas estáticas (`.a`) y paquetes Debian (`.deb`).

## Usage
- `ar x ruta/al/archivo.a` : extrae todos los miembros de un archivo
- `ar t ruta/al/archivo.ar` : lista los contenidos de un archivo
- `ar r ruta/al/archivo.deb ruta/a/binario-debian ruta/a/control.tar.gz ruta/a/datos.tar.xz ...` : reemplaza o añade archivos a un archivo
- `ar s ruta/al/archivo.a` : inserta un índice de archivo objeto (equivalente a usar `ranlib`)
- `ar rs ruta/al/archivo.a ruta/al/archivo1.o ruta/al/archivo2.o ...` : crea un archivo con archivos específicos incluyendo un índice de archivo objeto

## Links
- More information: https://manned.org/ar
