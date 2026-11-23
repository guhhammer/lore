# semanage fcontext

Gestiona las reglas persistentes del contexto de seguridad de SELinux en archivos y directorios.

## Usage
- `sudo semanage fcontext -l` : muestra todas las reglas de etiquetado de archivos
- `sudo semanage fcontext -lCn` : muestra todas las reglas de etiquetado de archivos definidas por el usuario sin encabezados
- `sudo semanage fcontext -a -t samba_share_t '/mnt/share(/.*)?'` : añade una regla definida por el usuario que etiquete cualquier ruta que coincida con una `expresión regular` PCRE
- `sudo semanage fcontext -a -e /ruta/a/ref /ruta/a/target` : añade una regla definida por el usuario que establezca una equivalencia de etiquetado entre dos subrutas
- `sudo semanage fcontext -d '/mnt/share(/.*)?'` : elimina una regla definida por el usuario utilizando su expresión regular PCRE
- `restorecon -Rv ruta/al/directorio` : reetiqueta un directorio de forma recursiva aplicando las nuevas reglas

## Links
- More information: https://manned.org/semanage-fcontext
