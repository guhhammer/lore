# blkpr

Registra, reserva, libera, anticipa y borra reservas persistentes en un dispositivo de bloque que soporte "Persistent Reservations".

## Usage
- `blkpr -c register -k clave_de_reserva ruta/al/dispositivo` : registra (comando) una nueva reserva con una clave dada en un dispositivo determinado
- `blkpr -c reserve -k clave_de_reserva -t exclusive-access ruta/al/dispositivo` : establece el tipo de reserva existente en acceso exclusivo
- `blkpr -c preempt -K clave_antigua -k nueva_clave -t write-exclusive ruta/al/dispositivo` : adelanta la reserva existente con una clave dada y la reemplaza por una nueva reserva
- `blkpr -c release -k clave_de_reserva -t tipo_de_reserva ruta/al/dispositivo` : libera una reserva con una clave y tipo dados en un dispositivo determinado
- `blkpr -c clear -k clave ruta/al/dispositivo` : borra todas las reservas de un dispositivo determinado

## Links
- More information: https://manned.org/blkpr
