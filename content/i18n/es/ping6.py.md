# ping6.py

Comprueba si se puede acceder a un host IPv6 mediante ICMPv6. Envía solicitudes de eco ICMPv6 y espera las respuestas de eco. Nota: Requiere privilegios de root para acceder a sockets sin procesar (por ejemplo, ejecútalo con `sudo`). Forma parte del conjunto de herramientas Impacket.

## Usage
- `ping6.py ipv6_origen ipv6_destino` : realiza un ping a un host IPv6 desde una dirección IPv6 de origen especificada
- `ping6.py 2001:db8::1 2001:db8::2` : envía un ping a 2001:db8::2 desde 2001:db8::1

## Links
- More information: https://github.com/fortra/impacket
