# semanage boolean

Manage persistent SELinux boolean settings.

## Usage
- `sudo semanage boolean -l` : list all booleans settings
- `sudo semanage boolean -l -C -n` : list all user-defined boolean settings without headings
- `sudo semanage boolean -m -1|--on|-0|--off haproxy_connect_any` : set or unset a boolean persistently

## See also
- getsebool
- setsebool

## Categories
users

## Links
- More information: https://manned.org/semanage-boolean
- tldr page: https://tldr.inbrowser.app/pages/linux/semanage-boolean
