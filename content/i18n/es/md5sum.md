# md5sum

Calcula sumas de comprobación criptográficas MD5.

## Usage
- `md5sum ruta/al/archivo1 ruta/al/archivo2 ...` : calcula la suma de comprobación MD5 de uno o más archivos
- `md5sum ruta/al/archivo1 ruta/al/archivo2 ... >ruta/al/archivo.md5` : calcula y guarda la lista de sumas de comprobación MD5 en un archivo
- `comando | md5sum` : calcula una suma de comprobación MD5 a partir de `stdin`
- `md5sum -c ruta/al/archivo.md5` : lee un archivo de sumas de comprobación MD5 y nombres de archivo y verifica que todos los archivos tengan sumas de comprobación coincidentes
- `md5sum -c --quiet ruta/al/archivo.md5` : muestra un mensaje solo para los archivos que faltan o cuando falla la verificación
- `md5sum --ignore-missing -c --quiet ruta/al/archivo.md5` : muestra un mensaje solo cuando falla la verificación, ignorando los archivos que faltan
- `echo suma_de_comprobación_md5_conocida_del_archivo ruta/al/archivo | md5sum -c` : comprueba la suma de comprobación MD5 conocida de un archivo
