# chown

Cambia la propiedad de usuario y grupo de archivos y directorios.

## Usage
- `chown usuario ruta/al/archivo_o_directorio` : cambia el usuario propietario de un archivo/directorio
- `chown usuario:grupo ruta/al/archivo_o_directorio` : cambia el usuario y grupo propietario de un archivo/directorio
- `chown usuario: ruta/al/archivo_o_directorio` : cambia el usuario propietario y el grupo para que ambos sean `usuario`
- `chown -R usuario ruta/al/directorio` : cambia recursivamente el propietario de un directorio y su contenido
- `chown -h usuario ruta/al/enlace_simbólico` : cambia el propietario de un enlace simbólico
- `chown --reference ruta/al/archivo_de_referencia ruta/al/archivo_o_directorio` : cambia el propietario de un archivo/directorio para que coincida con un archivo de referencia
