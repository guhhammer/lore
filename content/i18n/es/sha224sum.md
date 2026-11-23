# sha224sum

Calcula sumas de comprobación criptográficas SHA224.

## Usage
- `sha224sum ruta/al/archivo1 ruta/al/archivo2 ...` : calcula la suma de comprobación SHA224 para uno o más archivos
- `sha224sum ruta/al/archivo1 ruta/al/archivo2 ... >ruta/al/archivo.sha224` : calcula y guarda la lista de sumas de comprobación SHA224 en un archivo
- `comando | sha224sum` : calcula una suma de comprobación SHA224 desde `stdin`
- `sha224sum -c ruta/al/archivo.sha224` : lee nombres de archivo y archivo de sumas de comprobación SHA224 y verifica que todos los archivos tengan sumas de comprobación coincidentes
- `sha224sum -c --quiet ruta/al/archivo.sha224` : muestra solo un mensaje para los archivos que faltan o cuando falla la verificación
- `sha224sum --ignore-missing -c --quiet ruta/al/archivo.sha224` : muestra solo un mensaje cuando falla la verificación, ignorando los archivos que faltan
- `echo suma_de_comprobación_sha224_conocida_del_archivo ruta/al/archivo | sha224sum -c` : comprueba una suma de comprobación SHA224 conocida de un archivo

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/sha2-utilities.html
