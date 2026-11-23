# sockstat

Lista sockets abiertos de Internet o dominios UNIX. Nota: este programa es una reescritura para NetBSD 3.0 de `sockstat` de FreeBSD.

## Usage
- `sockstat` : muestra información de sockets IPv4, IPv6 y Unix que estén escuchando y conectados
- `sockstat -4|6 -l -P tcp|udp|sctp|divert -p puerto1,puerto2...` : muestra información para sockets IPv[4]/IPv[6] escuchando (listening) sobre puertos específicos usando un Protocolo específico
- `sockstat -cu` : también muestra sockets conectados, mostrando sockets unix
- `sockstat -n` : solo muestra salida numérica, sin resolver nombres simbólicos para direcciones y puertos
- `sockstat -f inet|inet6|local|unix` : lista sockets de una familia de direcciones específica

## Links
- More information: https://man.netbsd.org/sockstat.1
