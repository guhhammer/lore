# pathping

Una herramienta de traza de ruta que combina características de `ping` y `tracert`.

## Usage
- `pathping nombre_del_host` : realiza un ping y traza la ruta a un host
- `pathping nombre_del_host -n` : no realiza la búsqueda inversa de la dirección IP al nombre de host
- `pathping nombre_del_host -h max_hops` : especifica el número máximo de saltos para buscar el objetivo (el valor predeterminado es 30)
- `pathping nombre_del_host -p tiempo` : especifica los milisegundos a esperar entre pings (el valor predeterminado es 240)
- `pathping nombre_del_host -q consultas` : especifica el número de consultas por salto (el valor predeterminado es 100)
- `pathping nombre_del_host -4` : obliga al uso de IPV4
- `pathping nombre_del_host -6` : obliga al uso de IPV6
- `pathping /?` : muestra la ayuda
