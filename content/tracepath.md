# tracepath

Trace the path to a network host discovering MTU along this path.

## Usage
- `tracepath -p 33434 host` : trace the path to a host using a specific destination port
- `tracepath -p destination_port host` : specify the initial destination port, useful with non-standard firewall settings
- `tracepath -b host` : print both hostnames and numerical IP addresses
- `tracepath -m max_hops host` : specify a maximum TTL (number of hops)
- `tracepath -l packet_length host` : specify the initial packet length (defaults to 65535 for IPv4 and 128000 for IPv6)
- `tracepath -6 host` : use only IPv6 addresses

## Categories
network, files

## Links
- More information: https://manned.org/tracepath
- tldr page: https://tldr.inbrowser.app/pages/linux/tracepath
