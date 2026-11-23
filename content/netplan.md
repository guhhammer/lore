# netplan

Network configuration utility using YAML.

## Usage
- `sudo netplan apply` : apply a network configuration and make it persistent
- `sudo netplan generate` : generate backend configuration files
- `sudo netplan set ethernets.interface_name.dhcp4=true` : configure a network interface to use DHCP
- `sudo netplan try --timeout seconds` : try configuration changes without applying them permanently
- `sudo netplan --debug apply` : return to previous working configuration after failed apply
- `netplan status` : display the current netplan configuration status

## Categories
network, users, text

## Links
- More information: https://netplan.readthedocs.io/en/stable/cli/
- tldr page: https://tldr.inbrowser.app/pages/linux/netplan
