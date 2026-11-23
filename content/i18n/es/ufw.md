# ufw

Cortafuegos sin complicaciones (Uncomplicated Firewall). Interfaz de usuario de `iptables` para facilitar la configuración de un firewall.

## Usage
- `sudo ufw enable` : activa ufw
- `sudo ufw disable` : desactiva ufw
- `sudo ufw status numbered` : muestra reglas del ufw, junto con sus números
- `sudo ufw allow 5432 comment "servicio"` : permite el tráfico entrante en el puerto 5432 en este host con un comentario que identifique el servicio
- `sudo ufw allow proto tcp from 192.168.0.4 to any port 22` : permite sólo el tráfico TCP desde 192.168.0.4 a cualquier dirección de este host, en el puerto 22
- `sudo ufw deny 80` : deniega tráfico en el puerto 80 en este host
- `sudo ufw deny proto udp from any to any port 22` : deniega todo el tráfico al puerto 22
- `sudo ufw delete número_de_regla` : elimina una regla concreta. El número de la regla puede obtenerse del comando `ufw status numbered`

## Links
- More information: https://manned.org/ufw
