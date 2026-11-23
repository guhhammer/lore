# pacman

Arch Linux Paket Management Tool.

## Usage
- `sudo pacman -Syu` : synchronisiere und aktualisiere alle Pakete
- `sudo pacman -S paket` : installiere ein neues Paket
- `sudo pacman -Rs paket` : entferne ein Paket und dessen Abhängigkeiten
- `pacman -Q` : liste alle installierten Pakete und dessen Versionen auf
- `pacman -Qe` : liste alle ausdrücklich installierten Pakete und dessen Versionen auf
- `pacman -Qtdq` : zeige verwaiste Pakete an, welche als Abhängigkeiten installiert wurden, aber nicht mehr von anderen Paketen benötigt werden
- `sudo pacman -Scc` : leere den gesamten pacman Cache

## Links
- More information: https://manned.org/pacman.8
