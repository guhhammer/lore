# distccd

Server daemon for the distcc distributed compiler.

## Usage
- `distccd --daemon` : start a daemon with the default settings
- `distccd --daemon --allow-private` : start a daemon, accepting connections from IPv4 private network ranges
- `distccd --daemon -a ip_address|network_prefix` : start a daemon, accepting connections from a specific network address or address range
- `distccd --daemon -j 4 -N 5` : start a daemon with a lowered priority that can run a maximum of 4 tasks at a time
- `distccd --daemon --zeroconf` : start a daemon and register it via mDNS/DNS-SD (Zeroconf)

## Categories
system, network, development

## Links
- More information: https://manned.org/distccd
- tldr page: https://tldr.inbrowser.app/pages/common/distccd
