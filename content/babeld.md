# babeld

Routing daemon for Babel which uses firewall-style filters.

## Usage
- `babeld -c path/to/ports.conf -c path/to/filters.conf -c path/to/interfaces.conf` : start the daemon with one or more configuration files (read in order)
- `babeld -D` : daemonize after startup
- `babeld -C 'redistribute metric 256'` : specify a Configuration command
- `babeld eth0 eth1 wlan0` : specify on which interfaces to operate

## Categories
system, files, security

## Links
- More information: https://www.irif.fr/~jch/software/babel/babeld.html
- tldr page: https://tldr.inbrowser.app/pages/common/babeld
