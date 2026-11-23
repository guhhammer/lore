# ip maddress

Gestiona direcciones multicast.

## Usage
- `ip m` : lista las direcciones multicast y cuántos programas están suscritos a ellas
- `ip m s dev eth0` : lista de direcciones específicas de dispositivos
- `sudo ip m a 33:33:00:00:00:02 dev eth0` : se une a un grupo multicast estáticamente
- `sudo ip m d 33:33:00:00:00:02 dev eth0` : abandona un grupo multicast estático
- `ip m h` : muestra la ayuda

## Links
- More information: https://manned.org/ip-maddress
