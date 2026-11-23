# nmcli general

Manage general settings of NetworkManager.

## Usage
- `nmcli g` : show the general status of NetworkManager
- `nmcli g h` : show the hostname of the current device
- `sudo nmcli g h new_hostname` : change the hostname of the current device
- `nmcli g p` : show the permissions of NetworkManager
- `nmcli g l` : show the current logging level and domains
- `sudo nmcli g l l INFO|OFF|ERR|WARN|DEBUG|TRACE domain domain_1,domain_2,...` : set the logging level and/or domains (see `man NetworkManager.conf` for all available domains)

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/nmcli-general
