# rmlint

Encuentra desperdicio de espacio y otras cosas rotas en tu sistema de archivos.

## Usage
- `rmlint ruta/al/directorio1 ruta/al/directorio2 ...` : comprueba los directorios en busca de archivos duplicados, vacíos y rotos
- `rmlint -s 1MB ruta/al/directorio // ruta/al/directorio_original` : comprueba si hay duplicados mayores a un tamaño determinado, preferiblemente manteniendo los archivos en directorios etiquetados (después de la doble barra)
- `rmlint -K ruta/al/directorio // ruta/al/directorio_original` : comprueba que no se desperdicia espacio, manteniendo todo en los directorios no etiquetados
- `./rmlint.sh` : elimina archivos duplicados encontrados tras una ejecución de `rmlint`
- `rmlint -D ruta/al/directorio` : encuentra árboles de directorios duplicados basándose en los datos, ignorando los nombres
- `rmlint -S dl ruta/al/directorio` : marca archivos en la profundidad de la ruta inferior como originales, en caso de igualdad elegir uno más corto l
- `rmlint -c sh:link -b ruta/al/directorio` : encuentra archivos con idéntico nombre de archivo y contenido, y vincula en lugar de eliminar los duplicados
- `rmlint ruta/a/copia // ruta/a/datos -k -m` : utiliza `data` como directorio maestro. Busca solo los duplicados de la copia de seguridad que también estén en `datos`. No elimina ningún archivo de "datos"

## Links
- More information: https://rmlint.readthedocs.io/en/latest/rmlint.1.html
