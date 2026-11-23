# semanage boolean

Beheer persistente SELinux-boolean-instellingen.

## Usage
- `sudo semanage boolean -l` : toon alle boolean-instellingen
- `sudo semanage boolean -l -C -n` : toon alle door de gebruiker gedefinieerde boolean-instellingen zonder koppen
- `sudo semanage boolean -m -1|--on|-0|--off haproxy_connect_any` : stel een boolean persistent in of uit

## Links
- More information: https://manned.org/semanage-boolean
