# nmcli connection

Manage connections with NetworkManager.

## Usage
- `nmcli c` : list all NetworkManager connections (shows name, UUID, type, and device)
- `nmcli c u uuid` : activate a connection
- `nmcli c d uuid` : deactivate a connection
- `nmcli c a ifname interface_name type ethernet ipv4.method auto ipv6.method auto` : create an auto-configured dual stack connection
- `nmcli c a ifname interface_name type ethernet ip6 2001:db8::2/64 gw6 2001:db8::1 ipv6.dns 2001:db8::1 ipv4.method ignore` : create a static IPv6-only connection
- `nmcli c a ifname interface_name type ethernet ip4 10.0.0.7/8 gw4 10.0.0.1 ipv4.dns 10.0.0.1 ipv6.method ignore` : create a static IPv4-only connection
- `nmcli c i type openvpn file path/to/vpn_config.ovpn` : create a VPN connection using OpenVPN from an OVPN file

## Categories
files, network

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/nmcli-connection
