# netselect

Speed test for choosing a fast network server.

## Usage
- `sudo netselect host_1 host_2 ...` : choose the server with the lowest latency
- `sudo netselect -vv host_1 host_2 ...` : display nameserver resolution and statistics
- `sudo netselect -m 10 host_1 host_2 ...` : define maximum TTL (time to live)
- `sudo netselect -s n host_1 host_2 host_3 ...` : print `n` fastest servers among the hosts
- `netselect` : display help

## Categories
network, users, development

## Links
- More information: https://manned.org/netselect
- tldr page: https://tldr.inbrowser.app/pages/linux/netselect
