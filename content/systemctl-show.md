# systemctl show

Show properties of units or systemd itself.

## Usage
- `systemctl show` : show properties of the system service manager
- `systemctl show --user` : show properties of the user service manager
- `systemctl show unit` : show properties of a specific unit
- `systemctl show unit --user` : show properties of a specific user unit
- `systemctl show -a` : include empty properties in the list
- `systemctl show unit -p Wants,Conflicts,...` : only show the specified properties

## Categories
system, users

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/systemctl-show
