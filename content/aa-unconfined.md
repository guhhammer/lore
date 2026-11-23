# aa-unconfined

List processes with open TCP/UDP ports that do not have AppArmor profiles loaded.

## Usage
- `sudo aa-unconfined` : list unconfined processes using the `ss` command (default)
- `sudo aa-unconfined --with-netstat` : use `netstat` instead of `ss` to detect open network sockets
- `sudo aa-unconfined --paranoid` : show all processes from `/proc` with TCP/UDP ports and no AppArmor profiles (more detailed)
- `aa-unconfined -h` : display help

## Categories
processes, network, users

## Links
- More information: https://gitlab.com/apparmor/apparmor/-/wikis/manpage_aa-unconfined.8
- tldr page: https://tldr.inbrowser.app/pages/linux/aa-unconfined
