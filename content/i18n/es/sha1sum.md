# sha1sum

Calcula sumas de comprobación criptográficas SHA1.

## Usage
- `sha1sum ruta/al/archivo1 ruta/al/archivo2 ...` : calcula la suma de comprobación SHA1 de uno o más archivos
- `sha1sum ruta/al/archivo1 ruta/al/archivo2 ... >ruta/al/archivo.sha1` : calcula y guarda la lista de sumas de comprobación SHA1 en un archivo
- `comando | sha1sum` : calcula una suma de comprobación SHA1 a partir de `stdin`
- `sha1sum -c ruta/al/archivo.sha1` : lee un archivo de sumas de comprobación SHA1 y nombres de archivo y verifica que todos los archivos tengan sumas de comprobación coincidentes
- `sha1sum -c --quiet ruta/al/archivo.sha1` : solo muestra un mensaje cuando faltan archivos o cuando falla la verificación
- `sha1sum --ignore-missing -c --quiet ruta/al/archivo.sha1` : solo muestra un mensaje cuando falla la verificación, ignorando los archivos que faltan
- `echo suma_de_verificación_sha1_conocida_del_archivo ruta/al/archivo | sha1sum -c` : comprueba una suma de comprobación SHA1 conocida de un archivo
