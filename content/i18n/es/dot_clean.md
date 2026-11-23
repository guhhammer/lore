# dot_clean

Fusiona los archivos ._* con los archivos nativos correspondientes.

## Usage
- `dot_clean ruta/al/directorio` : fusiona todos los ficheros `._*` recursivamente
- `dot_clean -f ruta/al/directorio` : fusiona todos los `._*` en un directorio sin leer subdirectorios (fusión plana)
- `dot_clean -m ruta/al/directorio` : fusiona y elimina todos los archivos `._*`
- `dot_clean -n ruta/al/directorio` : elimina sólo los archivos `._*` si hay un archivo nativo coincidente
- `dot_clean -s ruta/al/directorio` : sigue los enlaces simbólicos
- `dot_clean -v ruta/al/directorio` : muestra resultados detallados

## Links
- More information: https://keith.github.io/xcode-man-pages/dot_clean.1.html
