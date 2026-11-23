# update-rc.d

Install and remove services which are System-V style init script links. Init scripts are in the `/etc/init.d/`.

## Usage
- `update-rc.d mysql defaults` : install a service
- `update-rc.d mysql enable` : enable a service
- `update-rc.d mysql disable` : disable a service
- `update-rc.d -f mysql remove` : forcibly remove a service

## Categories
system, databases, packages

## Links
- More information: https://manned.org/update-rc.d
- tldr page: https://tldr.inbrowser.app/pages/linux/update-rc.d
