# systemd-sysext

Activeer of deactiveer systeemextensie-images.

## Usage
- `systemd-sysext list` : toon geïnstalleerde extensie-images
- `systemd-sysext merge` : voeg systeemextensie-images samen in `/usr/` en `/opt/`
- `systemd-sysext` : toon de huidige status van het samenvoegen
- `systemd-sysext unmerge` : draai het samenvoegen van alle huidig geïnstalleerde systeemextensie-images terug in `/usr/` en `/opt/`
- `systemd-sysext refresh` : ververs de systeemextensie-images (een combinatie van `unmerge` en `merge`)

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/systemd-sysext.html
