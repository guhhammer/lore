# pathping

A trace route tool combining features of `ping` and `tracert`.

## Usage
- `pathping hostname` : ping and trace the route to a host
- `pathping hostname -n` : do not perform reverse lookup of IP address to hostname
- `pathping hostname -h max_hops` : specify the maximum number of hops to search for the target (the default is 30)
- `pathping hostname -p time` : specify the milliseconds to wait between pings (the default is 240)
- `pathping hostname -q queries` : specify the number of queries per hop (the default is 100)
- `pathping hostname -4` : force IPv4 usage
- `pathping hostname -6` : force IPv6 usage
- `pathping /?` : display help

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
network, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/pathping
