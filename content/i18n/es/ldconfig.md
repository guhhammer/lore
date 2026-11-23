# ldconfig

Configura enlaces simbólicos y caché para dependencias de biblioteca compartidas.

## Usage
- `sudo ldconfig` : actualiza los enlaces simbólicos y reconstruye el caché (normalmente se ejecuta cuando se instala una nueva biblioteca)
- `sudo ldconfig -n ruta/al/directorio` : actualiza los enlaces simbólicos para un directorio dado
- `ldconfig -p | grep nombre_de_biblioteca` : imprime las bibliotecas en el caché y comprueba si una biblioteca dada está presente

## Links
- More information: https://manned.org/ldconfig
