# setsebool

Stel de SELinux-boolean-waarde in.

## Usage
- `getsebool -a` : toon de huidige instelling van alle booleans
- `sudo setsebool httpd_can_network_connect 1|true|on|0|false|off` : stel een boolean tijdelijk in of uit (niet-persistent bij herstart)
- `sudo setsebool -P container_use_devices 1|true|on|0|false|off` : stel een boolean Persistent in of uit
- `sudo setsebool -P key1 1 key2 0 ...` : stel meerdere booleans tegelijk Persistent in of uit
- `sudo semanage boolean -m -1|--on|-0|--off haproxy_connect_any` : stel een boolean persistent in of uit (alternatieve methode met `semanage-boolean`)

## Links
- More information: https://manned.org/setsebool
