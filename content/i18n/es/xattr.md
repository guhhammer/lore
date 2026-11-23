# xattr

Utilidad para trabajar con atributos extendidos del sistema de ficheros.

## Usage
- `xattr -l archivo` : lista atributos extendidos clave:valor para un archivo dado
- `xattr -w atributo_clave atributo_valor archivo` : escribe un atributo para un archivo determinado
- `xattr -d com.apple.quarantine archivo` : elimina un atributo de un archivo determinado
- `xattr -c archivo` : elimina todos los atributos extendidos de un archivo determinado
- `xattr -rd clave_atributo ruta/al/directorio` : elimina recursivamente un atributo en un directorio determinado

## Links
- More information: https://keith.github.io/xcode-man-pages/xattr.1.html
