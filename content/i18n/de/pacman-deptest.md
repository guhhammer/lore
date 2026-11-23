# pacman-deptest

Überprüfe alle angegebenen Abhängigkeiten und gib eine Liste von Abhängigkeiten zurück, welche auf dem System nicht erfüllt sind.

## Usage
- `pacman -T paket1 paket2 ...` : zeige Paketnamen von Abhängigkeiten an, welche nicht installiert sind
- `pacman -T "bash>=5"` : überprüfe ob ein installiertes Paket eine Minimalversion erfüllt
- `pacman -T "bash>5"` : überprüfe ob eine neuere version eines Paketes installiert ist
- `pacman -Th` : zeige Hilfe an

## Links
- More information: https://manned.org/pacman.8
