# semanage

Herramienta de gestión de políticas persistentes de SELinux. Algunos subcomandos, como `boolean`, `fcontext`, `port`, etc., tienen su propia documentación de uso.

## Usage
- `sudo semanage boolean -m --on|--off haproxy_connect_any` : establece o desactiva un booleano de SELinux. Los booleanos permiten al administrador personalizar cómo las reglas de política afectan a los tipos de procesos confinados (también conocidos como dominios)
- `sudo semanage fcontext -a -t samba_share_t '/mnt/share(/.*)?'` : añade una regla de etiquetado de contexto de archivo definida por el usuario. Los contextos de archivo definen a qué archivos pueden acceder los dominios confinados
- `sudo semanage port -a -t ssh_port_t -p tcp 22000` : añade una regla de etiquetado de puertos definida por el usuario. Las etiquetas de puerto definen en qué puertos se permite que los dominios confinados escuchen
- `sudo semanage permissive --add|--delete httpd_t` : activa o desactiva el modo permisivo para un dominio confinado. El modo permisivo por dominio permite un control más granular en comparación con `setenforce`
- `sudo semanage export -f ruta/al/archivo` : muestra las personalizaciones locales en el almacén predeterminado
- `sudo semanage import -f ruta/al/archivo` : importa un archivo generado por `semanage export` a las personalizaciones locales (¡ATENCIÓN: puede eliminar las personalizaciones actuales!)

## Links
- More information: https://manned.org/semanage
