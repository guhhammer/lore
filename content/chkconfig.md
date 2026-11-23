# chkconfig

Manage the runlevel of services on CentOS 6.

## Usage
- `chkconfig --list` : list services with runlevel
- `chkconfig --list ntpd` : show a service's runlevel
- `chkconfig sshd on` : enable service at boot
- `chkconfig --level 2345 sshd on` : enable service at boot for runlevels 2, 3, 4, and 5
- `chkconfig ntpd off` : disable service at boot
- `chkconfig --level 3 ntpd off` : disable service at boot for runlevel 3

## Categories
system

## Links
- More information: https://manned.org/chkconfig
- tldr page: https://tldr.inbrowser.app/pages/linux/chkconfig
