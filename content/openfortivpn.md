# openfortivpn

A VPN client, for Fortinet's proprietary PPP+SSL VPN solution.

## Usage
- `openfortivpn -u username -p password` : connect to a VPN with a username and password
- `sudo openfortivpn -c path/to/config` : connect to a VPN using a specific configuration file (defaults to `/etc/openfortivpn/config`)
- `openfortivpn host:port` : connect to a VPN by specifying the host and port
- `openfortivpn --trusted-cert sha256_sum` : trust a given gateway by passing in its certificate's sha256 sum

## Categories
network, security, users

## Links
- More information: https://manned.org/openfortivpn
- tldr page: https://tldr.inbrowser.app/pages/linux/openfortivpn
