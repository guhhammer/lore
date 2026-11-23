# fastd

VPN daemon. Arbeitet auf Schicht 2 oder Schicht 3, unterstützt verschiedene Verschlüsselungsmethoden, wird von Freifunk verwendet.

## Usage
- `fastd -c pfad/zu/fastd.conf` : starte `fastd` mit einer bestimmten Konfigurationsdatei
- `fastd -m tap -M 1400 -c pfad/zu/fastd.conf` : starte einen Schicht-3-VPN mit einer MTU von 1400 und lade den Rest der Konfigurationsparameter aus einer Datei
- `fastd --verify-config -c pfad/zu/fastd.conf` : validiere eine Konfigurationsdatei
- `fastd --generate-key` : generiere einen neuen Schlüssel
- `fastd --show-key -c pfad/zu/fastd.conf` : zeige den öffentlichen Schlüssel zu einem privaten Schlüssel in einer Konfigurationsdatei an
- `fastd -v` : zeige Version an

## Links
- More information: https://fastd.readthedocs.io/en/stable/
