# tracert

Receive information about each step in the route between your PC and the target.

## Usage
- `tracert ip_address` : trace a route
- `tracert /d ip_address` : prevent `tracert` from resolving IP addresses to hostnames
- `tracert /4 ip_address` : force `tracert` to use IPv4 only
- `tracert /6 ip_address` : force `tracert` to use IPv6 only
- `tracert /h max_hops ip_address` : specify the maximum number of hops in the search for the target
- `tracert /?` : display help

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/tracert
