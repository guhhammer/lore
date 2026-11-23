# nmcli connection

Gestiona conexiones con NetworkManager. Este subcomando puede invocarse también con `nmcli c`.

## Usage
- `nmcli c` : lista todas las conexiones NetworkManager (muestra nombre, UUID, tipo y dispositivo)
- `nmcli c u uuid` : activa una conexión
- `nmcli c d uuid` : desactiva una conexión
- `nmcli c a ifname nombre_de_la_interfaz type ethernet ipv4.method auto ipv6.method auto` : crea una conexión de doble pila autoconfigurada
- `nmcli c a ifname nombre_de_la_interfaz type ethernet ip6 2001:db8::2/64 gw6 2001:db8::1 ipv6.dns 2001:db8::1 ipv4.method ignore` : crea una conexión estática exclusivamente IPv6
- `nmcli c a ifname nombre_de_la_interfaz type ethernet ip4 10.0.0.7/8 gw4 10.0.0.1 ipv4.dns 10.0.0.1 ipv6.method ignore` : crea una conexión estática exclusivamente IPv4
- `nmcli c i type openvpn file ruta/a/vpn_config.ovpn` : crea una conexión VPN usando OpenVPN desde un archivo OVPN
