# tcpreplay

Replay network traffic stored in a `pcap` file.

## Usage
- `tcpreplay --listnics` : list available network interfaces
- `tcpreplay -i eth0 traffic.pcap` : replay traffic to interface
- `tcpreplay -i eth0 -v traffic.pcap` : replay traffic to interface and `stdout`
- `tcpreplay -i eth0 -t traffic.pcap` : replay traffic to interface as fast as possible
- `tcpreplay -i eth0 -M 10 traffic.pcap` : replay traffic to interface at given Mbps
- `tcpreplay -i eth0 -l num_times traffic.pcap` : replay traffic to interface several times

## Categories
network, files

## Links
- More information: https://tcpreplay.appneta.com/wiki/tcpreplay-man.html
- tldr page: https://tldr.inbrowser.app/pages/common/tcpreplay
