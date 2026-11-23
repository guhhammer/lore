# tor-resolve

Resolve a hostname to an IP address via tor.

## Usage
- `tor-resolve [-4|-5] [-v] [-x] [-p socksport] hostname [sockshost[:socksport]]` : the general form, as the manual gives it
- `tor-resolve -v` : display verbose output
- `tor-resolve -x` : perform a reverse lookup: get the PTR record for an IPv4 address
- `tor-resolve -5` : use the SOCKS5 protocol
- `tor-resolve -p socksport` : override the default SOCKS port without setting the hostname

## Categories
network

## Links
- Documentation: https://spec.torproject.org/socks-extensions
