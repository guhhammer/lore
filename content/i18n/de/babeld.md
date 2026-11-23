# babeld

Routing-Daemon für Babel, der Firewall-ähnliche Filter benutzt.

## Usage
- `babeld -c pfad/zu/babeld.conf` : starte `babeld` mit einer bestimmten Konfigurationsdatei
- `babeld -c pfad/zu/ports.conf -c pfad/zu/filters.conf -c pfad/zu/interfaces.conf` : starte `babeld` mit mehreren Konfigurationsdateien (in der Reihenfolge des Einlesens)
- `babeld -D` : starte `babeld` als Daemon
- `babeld -C 'redistribute metric 256'` : starte `babeld` und übergib einen Konfigurationsbefehl
- `babeld eth0 eth1 wlan0` : starte `babeld` und gib an, auf welchen Schnittstellen gearbeitet werden soll

## Links
- More information: https://www.irif.fr/~jch/software/babel/babeld.html
