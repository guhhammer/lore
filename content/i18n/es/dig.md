# dig

Utilidad de búsqueda DNS.

## Usage
- `dig +short example.com` : busca la(s) IP(s) asociada(s) a un nombre de host (registros A)
- `dig +noall +answer example.com` : muestra una respuesta detallada para un dominio dado (registros A)
- `dig +short example.com A|MX|TXT|CNAME|NS` : consulta un tipo de registro DNS específico asociado a un nombre de dominio determinado
- `dig +tls @1.1.1.1|8.8.8.8|9.9.9.9|... example.com` : especifica un servidor DNS alternativo para consultar y, opcionalmente, utiliza DNS sobre TLS (DoT)
- `dig -x 8.8.8.8` : realiza una búsqueda DNS inversa en una dirección IP (registro PTR)
- `dig +nssearch example.com` : busca servidores de nombres autoritativos para la zona y muestra los registros SOA
- `dig +trace example.com` : realiza consultas iterativas y muestra la ruta de rastreo completa para resolver un nombre de dominio
- `dig +tcp -p puerto @dns_servidor_ip example.com` : consulta un servidor DNS a través de un puerto no estándar utilizando el protocolo TCP

## Links
- More information: https://manned.org/dig
