# ip maddress

Manage multicast addresses.

## Usage
- `ip m` : list multicast addresses and how many programs are subscribed to them
- `ip m s dev ethX` : list device specific addresses
- `sudo ip m a 33:33:00:00:00:02 dev ethX` : join a multicast group statically
- `sudo ip m d 33:33:00:00:00:02 dev ethX` : leave a static multicast group
- `ip m h` : display help

## Categories
users

## Links
- More information: https://manned.org/ip-maddress
- tldr page: https://tldr.inbrowser.app/pages/linux/ip-maddress
- Wikipedia: https://en.wikipedia.org/wiki/Iproute2
