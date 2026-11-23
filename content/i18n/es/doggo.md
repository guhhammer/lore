# doggo

Cliente DNS para Humanos. Escrito en Golang.

## Usage
- `doggo example.com` : realiza una simple búsqueda DNS
- `doggo MX codeberg.org @1.1.1.2` : consulta registros MX usando un servidor de nombres específico
- `doggo example.com @https://dns.quad9.net/dns-query` : utiliza DNS sobre HTTPS
- `doggo example.com -J | jq '.responses[0].answers[].address'` : salida en formato JSON
- `doggo -x 8.8.4.4 --short` : realiza una búsqueda DNS inversa
