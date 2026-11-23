# pacman-upgrade

Narzędzie do zarządzania pakietami w Arch Linuksie.

## Usage
- `sudo pacman -U ścieżka/do/pakietu1.pkg.tar.zst ścieżka/do/pakietu2.pkg.tar.zst ...` : zainstaluj jeden lub więcej pakietów z plików
- `sudo pacman -U --noconfirm ścieżka/do/pakietu.pkg.tar.zst` : zainstaluj pakiet bez pytania
- `sudo pacman -U --overwrite ścieżka/do/pliku ścieżka/do/pakietu.pkg.tar.zst` : nadpisz pliki będące w konflikcie podczas instalacji pakietów
- `sudo pacman -Ud ścieżka/do/pakietu.pkg.tar.zst` : zainstaluj pakiet, pomijając sprawdzanie wersji zależności
- `pacman -Up ścieżka/do/pakietu.pkg.tar.zst` : wyświetl pakiety, na które wpływ miałaby komenda (nie instaluje żadnych pakietów)
- `pacman -Uh` : wyświetl pomoc

## Links
- More information: https://manned.org/pacman.8
