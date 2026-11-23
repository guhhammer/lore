# pacman-key

Skrypt opakowujący dla GnuPG używany do zarządzania pękiem kluczy pacmana.

## Usage
- `sudo pacman-key --init` : zainicjalizuj pęk kluczy pacmana
- `sudo pacman-key --populate` : dodaj domyślne klucze Arch Linuksa
- `pacman-key -l` : wyświetl klucze z pęku publicznego
- `sudo pacman-key -a ścieżka/do/klucza.gpg` : dodaj podane klucze
- `sudo pacman-key -r "uid|imię|email"` : pobierz klucz z serwera kluczy
- `pacman-key -f "uid|imię|email"` : wyświetl odcisk podanego klucza
- `sudo pacman-key --lsign-key "uid|imię|email"` : podpisz zaimportowany klucz lokalnie
- `sudo pacman-key -d "uid|imię|email"` : usuń podany klucz

## Links
- More information: https://manned.org/pacman-key
