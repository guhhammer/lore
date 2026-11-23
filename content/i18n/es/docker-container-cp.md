# docker container cp

Copia archivos o directorios entre los sistemas de archivos del host y del contenedor.

## Usage
- `docker cp ruta/al/archivo_o_directorio_en_el_host nombre_del_contenedor:ruta/al/archivo_o_directorio_en_el_contenedor` : copia un archivo o directorio del host a un contenedor
- `docker cp nombre_del_contenedor:ruta/al/archivo_o_directorio_en_el_contenedor ruta/al/archivo_o_directorio_en_el_host` : copia un archivo o directorio de un contenedor al host
- `docker cp -L ruta/al/enlace_simbolico_en_el_host nombre_del_contenedor:ruta/al/archivo_o_directorio_en_el_contenedor` : copia un archivo o directorio desde el host a un contenedor, siguiendo los enlaces simbólicos (copia directamente los archivos enlazados, no los enlaces simbólicos en sí)

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/cp/
