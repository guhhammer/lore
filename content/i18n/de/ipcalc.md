# ipcalc

Einfache Operationen und Berechnungen mit IP-Adressen und Netzwerken durchführen.

## Usage
- `ipcalc 192.168.0.1` : zeige Netzwerkinformationen für eine IP-Adresse an
- `ipcalc 192.168.0.1/24` : zeige Netzwerkinformationen unter Verwendung der CIDR-Notation an
- `ipcalc 192.168.0.1 255.255.255.0` : zeige Netzwerkinformationen unter Verwendung einer gepunkteten dezimalen Netzmaske an
- `ipcalc -b 192.168.0.1` : unterdrücke Bitweise Ausgabe
- `ipcalc -s size1 size2 size3 ... 192.168.0.1` : teil ein Netzwerk in Blöcke der angegebenen Größe auf
- `ipcalc -v` : zeige Version an

## Links
- More information: https://manned.org/ipcalc
