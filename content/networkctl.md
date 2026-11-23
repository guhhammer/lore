# networkctl

Query the status of network links. Manage the network configuration using `systemd-networkd`.

## Usage
- `networkctl` : list existing links with their status
- `networkctl status` : show an overall network status
- `sudo networkctl up interface1 interface2 ...` : bring network devices up
- `sudo networkctl down interface1 interface2 ...` : bring network devices down
- `sudo networkctl renew interface1 interface2 ...` : renew dynamic configurations (e.g. IP addresses received from a DHCP server)
- `sudo networkctl reload` : reload configuration files (`.netdev` and `.network`)
- `sudo networkctl reconfigure interface1 interface2 ...` : reconfigure network interfaces (if you edited the config, you need to call `networkctl reload` first)

## Categories
network, users, system

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/networkctl.html
- tldr page: https://tldr.inbrowser.app/pages/linux/networkctl
