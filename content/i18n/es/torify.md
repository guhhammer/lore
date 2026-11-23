# torify

Enruta el tráfico de red a través de la red Tor. Nota: Este comando está desfasado, y ahora es un empaquetador compatible con versiones anteriores de `torsocks`.

## Usage
- `torify comando` : enruta el tráfico a través de Tor
- `torify on|off` : activa o desactiva Tor en el intérprete de comandos
- `torify --shell` : crea un intérprete de órdenes con Tor activado
- `torify show` : checa si hay un intérprete de órdenes con Tor activado
- `torify -c archivo_de_configuración comando` : especifica el archivo de configuración de Tor
- `torify -P proxy comando` : usa un proxy Tor SOCKS específico
- `torify comando >ruta/al/archivo_de_salida` : redirige la salida a un archivo

## Links
- More information: https://manned.org/torify
