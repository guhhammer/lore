# chgrp

Cambia la propiedad de grupo de archivos y directorios.

## Usage
- `chgrp grupo ruta/al/archivo_o_directorio` : cambia el grupo propietario de un archivo/directorio
- `chgrp -R grupo ruta/al/directorio` : cambia recursivamente el grupo propietario de un directorio y su contenido
- `chgrp -h grupo ruta/al/symlink` : cambia el grupo propietario de un enlace simbólico
- `chgrp --reference ruta/al/archivo_de_referencia ruta/al/archivo_o_directorio` : cambia el grupo propietario de un archivo/directorio para que coincida con un archivo de referencia
