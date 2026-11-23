# trip

A network diagnostic tool. Combines the functionality of `traceroute` and `ping`. Designed to assist with the analysis of networking issues.

## Usage
- `sudo trip example.com` : trace the route to a host with default parameters
- `trip example.com --unprivileged` : trace without requiring elevated privileges (supported platforms only)
- `sudo trip example.com --ipv6` : trace using `IPv6` only
- `sudo trip example.com --protocol udp` : trace using the `udp` protocol
- `sudo trip example.com --protocol tcp --target-port 443` : use custom destination port `443` for `tcp` tracing
- `sudo trip example.com --protocol udp --source-port 5000` : use custom source port `5000` for `udp` tracing

## Categories
network, users

## Links
- More information: https://trippy.rs/reference/cli/
- tldr page: https://tldr.inbrowser.app/pages/common/trip
