# dnstracer

El comando dnstracer determina de dónde obtiene la información un servidor DNS.

## Usage
- `dnstracer www.example.com` : averigua de dónde obtuvo tu servidor DNS local la información sobre www.example.com
- `dnstracer -s dns.example.org www.example.com` : empieza con un DNS específico que ya conozcas
- `dnstracer -4 www.example.com` : consulta solo servidores IPv[4]
- `dnstracer -r 5 www.example.com` : vuelve a intentar cada solicitud 5 veces en caso de fallo
- `dnstracer -v www.example.com` : muestra todos los pasos durante la ejecución
- `dnstracer -o www.example.com` : muestra un resumen de todas las respuestas recibidas tras la ejecución

## Links
- More information: https://manned.org/dnstracer
