# upnpc

Configure port forwarding rules on your router via the UPnP protocol.

## Usage
- `upnpc -a 192.168.0.1 8080 80 tcp` : forward the external TCP port 80 to port 8080 on a local machine
- `upnpc -d 80 tcp` : delete any port redirection for external TCP port 80
- `upnpc -s` : get information about UPnP devices on your network
- `upnpc -l` : list existing redirections

## Categories
network

## Links
- More information: https://manned.org/upnpc
- tldr page: https://tldr.inbrowser.app/pages/linux/upnpc
