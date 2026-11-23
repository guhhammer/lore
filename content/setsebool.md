# setsebool

Set SELinux boolean value.

## Usage
- `getsebool -a` : show the current setting of all booleans
- `sudo setsebool httpd_can_network_connect 1|true|on|0|false|off` : set or unset a boolean temporarily (non-persistent across reboot)
- `sudo setsebool -P container_use_devices 1|true|on|0|false|off` : set or unset a boolean Persistently
- `sudo setsebool -P key1 1 key2 0 ...` : set or unset multiple booleans at once Persistently
- `sudo semanage boolean -m -1|--on|-0|--off haproxy_connect_any` : set or unset a boolean persistently (alternative method using `semanage-boolean`)

## See also
- semanage boolean
- getsebool

## Categories
users

## Links
- More information: https://manned.org/setsebool
- tldr page: https://tldr.inbrowser.app/pages/linux/setsebool
