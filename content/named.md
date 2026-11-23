# named

Execute the DNS (Dynamic Name Service) server daemon that converts host names to IP addresses and vice versa.

## Usage
- `named` : read the default configuration file `/etc/named.conf`, read any initial data, and listen for queries
- `named -c path/to/named.conf` : read a custom configuration file
- `named -4|-6` : use IPv4 or IPv6 only, even if the host machine is capable of utilizing other protocols
- `named -p port` : listen for queries on a specific port instead of the default port 53
- `named -f` : run the server in the foreground and do not daemonize

## Categories
network, system, files

## Links
- More information: https://manned.org/named
- tldr page: https://tldr.inbrowser.app/pages/linux/named
