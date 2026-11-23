# pacman-remove

Arch Linux Paketverwaltungs-Werkzeug.

## Usage
- `sudo pacman -Rs paketname` : entferne ein Paket und dessen Abhängigkeiten
- `sudo pacman -Rsn paketname` : entferne ein Paket sowie alle Abhängigkeiten und Konfigurationsdateien
- `sudo pacman -R --noconfirm paketname` : entferne ein Paket ohne Bestätigungsaufforderung
- `sudo pacman -Rsn $(pacman -Qdtq)` : entferne verwaiste Pakete (Pakete welche als Abhängigkeit installiert wurden, aber von keinem Paket benötigt werden)
- `sudo pacman -Rc paketname` : entferne ein Paket und alle Pakete die davon abhängig sind
- `pacman -Rp paketname` : liste Pakete auf, welche betroffen sein würden (entfernt keine Pakete)
- `pacman -Rh` : zeige Hilfe für diesen Unterbefehl an

## Links
- More information: https://manned.org/pacman.8
