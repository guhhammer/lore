# httpry

A lightweight packet sniffer for displaying and logging HTTP traffic. It can be run in real-time displaying the traffic as it is parsed, or as a daemon process that logs to an output file.

## Usage
- `httpry -o path/to/file.log` : save output to a file
- `httpry eth0 -b path/to/file.pcap` : listen on a specific interface and save output to a binary PCAP format file
- `httpry -m get|post|put|head|options|delete|trace|connect|patch` : filter output by a comma-separated list of HTTP verbs
- `httpry -r path/to/file.log 'host 192.168.5.25'` : read from an input capture file and filter by IP
- `httpry -d -o path/to/file.log` : run as daemon process

## Categories
files, network, processes

## Links
- More information: https://dumpsterventures.com/jason/httpry/
- tldr page: https://tldr.inbrowser.app/pages/common/httpry
