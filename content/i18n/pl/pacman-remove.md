# pacman-remove

Narzędzie do zarządzania pakietami w Arch Linuksie.

## Usage
- `sudo pacman -Rs nazwa_pakietu` : usuń pakiet i jego zależności
- `sudo pacman -Rsn nazwa_pakietu` : usuń pakiet, jego zależności i pliki konfiguracyjne
- `sudo pacman -R --noconfirm nazwa_pakietu` : usuń pakiet bez pytania
- `sudo pacman -Rsn $(pacman -Qdtq)` : usuń pakiety-sieroty (zainstalowane jako zależności ale nie są już wymagane przez żaden pakiet)
- `sudo pacman -Rc nazwa_pakietu` : usuń pakiet i wszystke pakiety, które od niego zależą
- `pacman -Rp nazwa_pakietu` : wyświetl pakiety, na które wpływ miałaby komenda (nie usuwa żadnych pakietów)
- `pacman -Rh` : wyświetl pomoc dla tej komendy

## Links
- More information: https://manned.org/pacman.8
