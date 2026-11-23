# pacman-upgrade

Arch Linux Paketverwaltungs-Werkzeug.

## Usage
- `sudo pacman -U pfad/zu/paket1.pkg.tar.zst pfad/zu/paket2.pkg.tar.zst ...` : installiere ein oder mehrere Pakete von Dateien
- `sudo pacman -U --noconfirm pfad/zu/paket.pkg.tar.zst` : installiere ein Paket ohne Bestätigungsaufforderung
- `sudo pacman -U --overwrite pfad/zu/datei pfad/zu/paket.pkg.tar.zst` : überschreibe widersprüchliche Dateien während einer Paketinstallation
- `sudo pacman -Ud pfad/zu/paket.pkg.tar.zst` : installiere ein Paket und überspringe die Überprüfung von Abhängigkeitsversionen
- `pacman -Up pfad/zu/paket.pkg.tar.zst` : liste Pakete auf, welche betroffen sein würden (installiert keine Pakete)
- `pacman -Uh` : zeige Hilfe an

## Links
- More information: https://manned.org/pacman.8
