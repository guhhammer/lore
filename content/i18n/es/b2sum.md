# b2sum

Calcula sumas de comprobación criptográficas BLAKE2.

## Usage
- `b2sum ruta/al/archivo1 ruta/al/archivo2 ...` : calcula la suma de comprobación BLAKE2 para uno o más archivos
- `b2sum ruta/al/archivo1 ruta/al/archivo2 ... >ruta/al/archivo.b2` : calcula y guarda la lista de sumas de comprobación BLAKE2 en un archivo
- `comando | b2sum` : calcula una suma de comprobación BLAKE2 desde `stdin`
- `b2sum -c ruta/al/archivo.b2` : lee un archivo de sumas de comprobación BLAKE2 y nombres de archivo y verifica que todos los archivos tengan sumas de comprobación coincidentes
- `b2sum -c --quiet ruta/al/archivo.b2` : muestra solo un mensaje para los archivos que faltan o cuando falla la verificación
- `b2sum --ignore-missing -c --quiet ruta/al/archivo.b2` : muestra solo un mensaje cuando falle la verificación, ignorando los archivos que faltan
- `echo suma_de_comprobación_blake2_conocida_del_archivo ruta/al/archivo | b2sum -c` : comprueba una suma de comprobación BLAKE2 conocida de un archivo
