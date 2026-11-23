# ping6

Send ICMP ECHO_REQUEST packets to network hosts via IPv6 address. Note: `ping6` has been merged to `ping` in modern systems.

## Usage
- `ping6 host` : ping a host
- `ping6 -c count host` : ping a host only a specific number of times
- `ping6 -i seconds host` : ping a host, specifying the interval in seconds between requests (default is 1 second)
- `ping6 -n host` : ping a host without trying to look up symbolic names for addresses
- `ping6 -a host` : ping a host and ring the bell when a packet is received (if your terminal supports it)

## Categories
network

## Links
- More information: https://manned.org/ping6
- tldr page: https://tldr.inbrowser.app/pages/common/ping6
