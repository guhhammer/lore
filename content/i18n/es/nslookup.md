# nslookup

Consulta los servidores de nombres para obtener diversos registros de dominio.

## Usage
- `nslookup example.com` : consulta al servidor de nombres predeterminado de tu sistema la dirección IP (registro A) del dominio
- `nslookup -type=NS example.com 8.8.8.8` : consulta un servidor de nombres determinado para obtener un registro NS del dominio
- `nslookup -type=PTR 54.240.162.118` : realiza una consulta de búsqueda inversa (registro PTR) de una dirección IP
- `nslookup -vc -type=ANY example.com` : consulta CUALQUIER registro disponible utilizando el protocolo TCP
- `nslookup -vc -type=AXFR example.com nombre_servidor` : consulta un servidor de nombres determinado para obtener el archivo de zona completo (transferencia de zona) del dominio utilizando el protocolo TCP
- `nslookup -type=MX -debug example.com` : consulta un servidor de correo (registro MX) del dominio, mostrando los detalles de la transacción
- `nslookup -port=número_puerto -type=TXT example.com nombre_servidor` : consulta un servidor de nombres determinado en un número de puerto específico para obtener un registro TXT del dominio

## Links
- More information: https://manned.org/nslookup
