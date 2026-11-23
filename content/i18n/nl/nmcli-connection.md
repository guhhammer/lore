# nmcli connection

Beheer verbindingen met NetworkManager.

## Usage
- `nmcli c` : toon alle NetworkManager-verbindingen (toont naam, UUID, type en apparaat)
- `nmcli c u uuid` : activeer een verbinding
- `nmcli c d uuid` : deactiveer een verbinding
- `nmcli c a ifname interface_naam type ethernet ipv4.method auto ipv6.method auto` : maak een automatisch geconfigureerde dual stack verbinding
- `nmcli c a ifname interface_naam type ethernet ip6 2001:db8::2/64 gw6 2001:db8::1 ipv6.dns 2001:db8::1 ipv4.method ignore` : maak een statische IPv6-only verbinding
- `nmcli c a ifname interface_naam type ethernet ip4 10.0.0.7/8 gw4 10.0.0.1 ipv4.dns 10.0.0.1 ipv6.method ignore` : maak een statische IPv4-only verbinding
- `nmcli c i type openvpn file pad/naar/vpn_config.ovpn` : maak een VPN-verbinding via OpenVPN vanuit een OVPN bestand
