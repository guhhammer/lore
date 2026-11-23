# chmod

Cambia los permisos de acceso de un archivo o directorio.

## Usage
- `chmod u+x ruta/al/archivo` : otorga al usuario que es propietario del archivo permiso para x ejecutarlo
- `chmod u+rw ruta/al/archivo_o_directorio` : otorga al usuario derechos para leer (r) y escribir (w) un archivo o directorio
- `chmod g-x ruta/al/archivo` : elimina los derechos de ejecución del grupo
- `chmod a+rx ruta/al/archivo` : otorga a todos los usuarios (a) derechos para leer y ejecutar
- `chmod o=g ruta/al/archivo` : otorga a otros (que no están en el grupo del propietario) los mismos derechos que los del grupo
- `chmod o= ruta/al/archivo` : quita todos los derechos a otros
- `chmod -R g+w,o+w ruta/al/directorio` : otorga al grupo y a otros el derecho para escribir (w) un directorio y su contenido
- `chmod -R a+rX ruta/al/directorio` : concede de forma recursiva a todos los usuarios permisos de lectura a los archivos y permisos de eXecución a los subdirectorios dentro de un directorio
