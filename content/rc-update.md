# rc-update

Add and remove OpenRC services to and from runlevels.

## Usage
- `rc-update` : list enabled services and the runlevels they are added to
- `rc-update -v` : list all services
- `sudo rc-update add service_name runlevel` : add a service to a runlevel
- `sudo rc-update del service_name runlevel` : delete a service from a runlevel
- `sudo rc-update -a del service_name` : delete a service from all runlevels

## See also
- openrc

## Categories
system, users

## Links
- More information: https://manned.org/rc-update
- tldr page: https://tldr.inbrowser.app/pages/linux/rc-update
