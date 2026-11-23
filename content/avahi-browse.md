# avahi-browse

Display services and hosts exposed on the local network via mDNS/DNS-SD. Avahi is compatible with Bonjour (Zeroconf) found in Apple devices.

## Usage
- `avahi-browse -a -r -l` : list services available on the local network along with their addresses and ports, ignoring ones on the local machine
- `avahi-browse -a -t -p` : quickly list services in the local network in SSV format for scripts
- `avahi-browse -D` : list domains in the neighborhood
- `avahi-browse -a --domain=domain` : limit the search to a particular domain

## Categories
network, system, desktop

## Links
- More information: https://manned.org/avahi-browse
- tldr page: https://tldr.inbrowser.app/pages/linux/avahi-browse
