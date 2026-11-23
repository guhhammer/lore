# ethtool

Display and modify Network Interface Controller (NIC) parameters.

## Usage
- `ethtool eth0` : display the current settings for an interface
- `ethtool -i eth0` : display the driver information for an interface
- `ethtool -k eth0` : display all supported features for an interface
- `ethtool -S eth0` : display the network usage statistics for an interface
- `ethtool -p eth0 10` : blink one or more LEDs on an interface for 10 seconds
- `ethtool -s eth0 speed 10|100|1000 duplex half|full autoneg on|off` : set the link speed, duplex mode, and parameter auto-negotiation for a given interface

## Categories
network, desktop

## Links
- More information: https://manned.org/ethtool
- tldr page: https://tldr.inbrowser.app/pages/linux/ethtool
