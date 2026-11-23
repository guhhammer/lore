# wg-quick

Quickly set up WireGuard tunnels based on config files.

## Usage
- `sudo wg-quick up interface` : set up a VPN tunnel based on `/etc/wireguard/interface.conf`
- `sudo wg-quick down interface` : delete a VPN tunnel
- `sudo wg-quick save interface` : save the current config of a tunnel to `/etc/wireguard/interface.conf`
- `sudo wg-quick strip interface` : print the config of a tunnel with `wg-quick`-specific options removed

## Categories
network, users

## Links
- More information: https://manned.org/wg-quick
- tldr page: https://tldr.inbrowser.app/pages/common/wg-quick
