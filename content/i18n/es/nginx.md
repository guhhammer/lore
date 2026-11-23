# nginx

Servidor web `nginx`.

## Usage
- `nginx` : inicia el servidor con el archivo de configuración por defecto
- `nginx -c archivo_de_configuración` : inicia el servidor con un archivo de configuración personalizado
- `nginx -c archivo_de_configuración -p prefijo/para/rutas/relativas` : inicia el servidor con un prefijo para todas las rutas relativas en el archivo de configuración
- `nginx -t` : prueba la configuración sin afectar al servidor en ejecución
- `nginx -s reload` : recarga la configuración enviando una señal sin tiempo de inactividad

## Links
- More information: https://nginx.org/docs/switches.html
