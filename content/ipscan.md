# ipscan

A fast network scanner designed to be simple to use. Also known as Angry IP Scanner.

## Usage
- `ipscan 192.168.0.1` : scan a specific IP address
- `ipscan 192.168.0.1-254` : scan a range of IP addresses
- `ipscan 192.168.0.1-254 -o path/to/output.txt` : scan a range of IP addresses and save the results to a file
- `ipscan 192.168.0.1-254 -p 80,443,22` : scan IPs with a specific set of ports
- `ipscan 192.168.0.1-254 -d 200` : scan with a delay between requests to avoid network congestion
- `ipscan --help` : display help

## Categories
network

## Links
- More information: https://www.aldeid.com/wiki/Angry-IPScan#CLI
- tldr page: https://tldr.inbrowser.app/pages/common/ipscan
