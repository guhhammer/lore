# zeek

Passive network traffic analyzer. Any output and log files will be saved to the current working directory.

## Usage
- `sudo zeek --iface interface` : analyze live traffic from a network interface
- `sudo zeek --iface interface script1 script2 ...` : analyze live traffic from a network interface and load custom scripts
- `sudo zeek --bare-mode --iface interface` : analyze live traffic from a network interface, without loading any scripts
- `sudo zeek --filter path/to/filter --iface interface` : analyze live traffic from a network interface, applying a `tcpdump` filter
- `sudo zeek --watchdog --iface interface` : analyze live traffic from a network interface using a watchdog timer
- `zeek --readfile path/to/file.trace` : analyze traffic from a PCAP file

## Categories
network, files, users

## Links
- More information: https://docs.zeek.org/en/lts/quickstart.html#zeek-as-a-command-line-utility
- tldr page: https://tldr.inbrowser.app/pages/common/zeek
