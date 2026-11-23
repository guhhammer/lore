# apache2ctl

Administrate the Apache HTTP web server. This command comes with Debian based OSes, for RHEL based ones see `httpd`.

## Usage
- `sudo apache2ctl start` : start the Apache daemon. Throw a message if it is already running
- `sudo apache2ctl stop` : stop the Apache daemon
- `sudo apache2ctl restart` : restart the Apache daemon
- `sudo apache2ctl -t` : test syntax of the configuration file
- `sudo apache2ctl -M` : list loaded modules

## Categories
network, users, system

## Links
- More information: https://manned.org/apache2ctl
- tldr page: https://tldr.inbrowser.app/pages/linux/apache2ctl
