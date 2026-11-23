# mtr

Matt's Traceroute: herramienta que combina traceroute y ping.

## Usage
- `mtr example.com` : realiza un traceroute a un host y envía pings de forma continua a todos los saltos intermedios
- `mtr -n example.com` : desactiva la resolución de direcciones IP y nombres de host
- `mtr -w example.com` : genera un informe tras enviar 10 pings a cada salto
- `mtr -4 example.com` : fuerza IPv4 o IPv6
- `mtr -i 10 example.com` : espera un tiempo determinado (en segundos) antes de enviar otro paquete al mismo salto
- `mtr -z example.com` : muestra el número de sistema autónomo (ASN) de cada salto
- `mtr -b example.com` : muestra tanto la dirección IP como el nombre DNS inverso

## Links
- More information: https://manned.org/mtr
