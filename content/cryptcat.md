# cryptcat

Netcat fork with encryption capabilities.

## Usage
- `cryptcat -k password -l -p port` : listen on a specified port and print any data received
- `cryptcat -k password ip_address port` : connect to a certain port
- `cryptcat -k password -w timeout_in_seconds ip_address port` : specify the timeout (w)
- `cryptcat -v -z ip_address port` : scan (z) the open ports of a specified host
- `cryptcat -k password -l -p local_port | cryptcat -k password hostname remote_port` : act as proxy and forward data from a local TCP port to the given remote host

## Categories
network, users

## Links
- More information: https://manned.org/cryptcat
- tldr page: https://tldr.inbrowser.app/pages/linux/cryptcat
