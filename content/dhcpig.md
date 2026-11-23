# dhcpig

Initiate an advanced DHCP exhaustion attack and stress test.

## Usage
- `sudo path/to/pig.py eth0` : exhaust all of the available DHCP addresses using the specified interface
- `sudo path/to/pig.py -6 eth1` : exhaust IPv6 addresses using eth1 interface
- `sudo path/to/pig.py -f eth1` : send fuzzed/malformed data packets using the interface
- `sudo path/to/pig.py -c eth1` : enable color output
- `sudo path/to/pig.py -c -v 1 eth1` : enable minimal verbosity and color output
- `sudo path/to/pig.py -c -v 100 -n eth1` : use a debug verbosity of 100 and scan network of neighboring devices using ARP packets
- `sudo path/to/pig.py -n -r -o eth1` : enable printing lease information, attempt to scan and release all neighbor IP addresses

## Categories
files, users, network

## Links
- More information: https://github.com/kamorin/DHCPig#usage
- tldr page: https://tldr.inbrowser.app/pages/common/dhcpig
