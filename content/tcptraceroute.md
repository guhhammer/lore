# tcptraceroute

A traceroute implementation using TCP packets.

## Usage
- `tcptraceroute host` : trace the route to a host
- `tcptraceroute host destination_port packet_length` : specify the destination port and packet length in bytes
- `tcptraceroute host -p source_port -s source_address` : specify the local source port and source address
- `tcptraceroute host -f first_ttl -m max_ttl` : set the first and maximum TTL
- `tcptraceroute host -w wait_time -q number_of_queries` : specify the wait time and number of queries per hop
- `tcptraceroute host -i interface` : specify the interface

## Categories
network

## Links
- More information: https://manned.org/tcptraceroute
- tldr page: https://tldr.inbrowser.app/pages/linux/tcptraceroute
