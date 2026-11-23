# zoneadm

Beheer Oracle Solaris zones.

## Usage
- `zoneadm list -cv` : toon alle zones en hun huidige status
- `sudo zoneadm -z zone_naam verify` : verifieer de configuratie van een specifieke zone
- `sudo zoneadm -z zone_naam install` : installeer een zone
- `sudo zoneadm -z zone_naam boot` : boot (start) een zone
- `sudo zoneadm -z zone_naam reboot` : herstart een zone
- `sudo zoneadm -z zone_naam halt` : halt (stop) een zone, waarbij eventuele afsluitscripts binnen de zone worden overgeslagen
- `sudo zoneadm -z zone_naam uninstall` : verwijder een zone

## Links
- More information: https://docs.oracle.com/cd/E88353_01/html/E72487/zoneadm-8.html
