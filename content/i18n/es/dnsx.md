# dnsx

Un paquete de herramientas DNS rápido y polivalente para ejecutar múltiples consultas DNS. Nota: En algunos casos, la entrada a `dnsx` debe pasarse a través de `stdin` ( pipe `|`).

## Usage
- `echo example.com | dnsx -a -re` : consulta el registro A de un (sub)dominio y muestra la respuesta recibida
- `dnsx <<<example.com -recon -re` : consulta todos los registros DNS (A, AAAA, CNAME, NS, TXT, SRV, PTR, MX, SOA, AXFR, CAA)
- `echo example.com | dnsx -re -a|aaaa|cname|ns|txt|srv|ptr|mx|soa|any|axfr|caa` : consulta un tipo específico de registro DNS
- `echo example.com | dnsx -ro` : muestra solo la respuesta (sin mostrar el dominio o subdominio consultado)
- `echo example.com | dnsx -debug|raw -r 1.1.1.1,8.8.8.8,... -retry número` : muestra la respuesta sin procesar de una consulta, especificando los resolutores que se van a utilizar y los intentos de reintento en caso de fallos
- `dnsx -d FUZZ.example.com -w ruta/a/wordlist.txt -re` : ataque de fuerza bruta a registros DNS utilizando un marcador de posición
- `dnsx -d ruta/a/dominio.txt -w ruta/a/lista_de_palabras.txt -re -o ruta/a/salida.txt -nc` : utiliza fuerza bruta en los registros DNS a partir de una lista de dominios y listas de palabras, añadiendo la salida a un archivo sin códigos de color
- `subfinder -silent -d example.com | dnsx -cname -re -rl number` : extrae registros `CNAME` para la lista dada de subdominios, con limitación de la tasa de consultas DNS por segundo

## Links
- More information: https://docs.projectdiscovery.io/opensource/dnsx/usage
