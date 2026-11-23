# netsh interface portproxy

Configure and display the status of various network components.

## Usage
- `netsh interface portproxy show all` : display the current port forwarding setup
- `netsh interface portproxy add v4tov4 listenaddress=192.168.0.1 listenport=8080 connectaddress=10.0.0.1 connectport=80` : set up IPv4 port forwarding (run in elevated console)
- `netsh interface portproxy delete v4tov4 listenaddress=192.168.0.1 listenport=8080` : remove IPv4 port forwarding (run in elevated console)
- `netsh interface portproxy` : display help

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
network, desktop, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/netsh-interface-portproxy
