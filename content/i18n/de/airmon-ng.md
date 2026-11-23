# airmon-ng

Aktiveren des Überwachungsmodus auf Wireless Network Geräten. Teil von `aircrack-ng`.

## Usage
- `sudo airmon-ng` : liste Wireless Geräte und deren Status auf
- `sudo airmon-ng start wlan0` : aktiviere den Überwachungsmodus für ein bestimmtes Gerät
- `sudo airmon-ng check kill` : kille störende Prozesse, die das Wireless Gerät verwenden
- `sudo airmon-ng stop wlan0mon` : deaktiviere den Überwachungsmodus für ein spezifisches Interface

## Links
- More information: https://www.aircrack-ng.org/doku.php?id=airmon-ng
