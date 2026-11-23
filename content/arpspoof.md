# arpspoof

Forge ARP replies to intercept packets.

## Usage
- `sudo arpspoof -i wlan0 host_ip` : poison all hosts to intercept packets on interface for the host
- `sudo arpspoof -i wlan0 -t target_ip host_ip` : poison target to intercept packets on interface for the host
- `sudo arpspoof -i wlan0 -r -t target_ip host_ip` : poison both target and host to intercept packets on interface for the host

## Categories
network, users

## Links
- More information: https://manned.org/arpspoof
- tldr page: https://tldr.inbrowser.app/pages/linux/arpspoof
