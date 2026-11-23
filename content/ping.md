# ping

Send ICMP ECHO_REQUEST packets to network hosts.

## Usage
- `ping 10.0.0.1` : ping a host with IP `10.0.0.1` (Note: `10.0.0.1` can be shortened to `10.1`)
- `ping -c count host` : ping a host only a specific number of times
- `ping -i seconds host` : ping host, specifying the interval in seconds between requests (default is 1 second)
- `ping -n host` : ping host without trying to look up symbolic names for addresses
- `ping -a host` : ping host and ring the bell when a packet is received (if your terminal supports it)
- `ping -O host` : also display a message if no response was received
- `ping -c count -W seconds -w seconds host` : ping a host with specific count of pings, per-packet time to Wait for a response, and total wait time limit of the entire ping run
- `ping -6 ff02::1%eth0` : ping all IPv6 local network hosts

## See also
- mtr

## Categories
network

## Links
- More information: https://manned.org/ping
- tldr page: https://tldr.inbrowser.app/pages/common/ping
