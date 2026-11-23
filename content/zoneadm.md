# zoneadm

Administer Oracle Solaris zones.

## Usage
- `zoneadm list -cv` : list all zones and their current status
- `sudo zoneadm -z zone_name verify` : verify the configuration of a specific zone
- `sudo zoneadm -z zone_name install` : install a zone
- `sudo zoneadm -z zone_name boot` : boot (start) a zone
- `sudo zoneadm -z zone_name reboot` : reboot a zone
- `sudo zoneadm -z zone_name halt` : halt (stop) a zone, bypassing any shutdown scripts inside the zone
- `sudo zoneadm -z zone_name uninstall` : uninstall a zone

## Notes
A SunOS (Solaris) command: this page is for reference, it does not run on Linux or macOS.

## Categories
users, system, sunos

## Links
- More information: https://docs.oracle.com/cd/E88353_01/html/E72487/zoneadm-8.html
- tldr page: https://tldr.inbrowser.app/pages/sunos/zoneadm
