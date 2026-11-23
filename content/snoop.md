# snoop

Network packet sniffer. SunOS equivalent of `tcpdump`.

## Usage
- `snoop -d e1000g0` : capture packets on a specific network interface
- `snoop -o path/to/file` : save captured packets in a file instead of displaying them
- `snoop -V -i path/to/file` : display verbose protocol layer summary of packets from a file
- `snoop to port port from host hostname` : capture network packets that come from a hostname and go to a given port
- `snoop -x0 -p4 ip1 ip2` : capture and show a hex-dump of network packets exchanged between two IP addresses

## Notes
A SunOS (Solaris) command: this page is for reference, it does not run on Linux or macOS.

## Categories
network, files, sunos

## Links
- More information: https://www.unix.com/man-page/sunos/1m/snoop
- tldr page: https://tldr.inbrowser.app/pages/sunos/snoop
