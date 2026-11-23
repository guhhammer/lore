# arpspoof

Forja respuestas ARP para interceptar paquetes.

## Usage
- `sudo arpspoof -i wlan0 ip_del_host` : envenena todos los hosts para interceptar paquetes en la interfaz para el host
- `sudo arpspoof -i wlan0 -t ip_del_objetivo ip_del_host` : envenena el objetivo para interceptar paquetes en la interfaz para el host
- `sudo arpspoof -i wlan0 -r -t ip_del_objetivo ip_del_host` : envenena tanto el objetivo como el host para interceptar paquetes en la interfaz para el host

## Links
- More information: https://manned.org/arpspoof
