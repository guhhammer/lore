# pacman

Narzędzie do zarządzania pakietami w Arch Linuksie.

## Usage
- `sudo pacman -Syu` : zsynchronizuj i zaktualizuj wszystkie pakiety
- `sudo pacman -S pakiet` : zainstaluj nowy pakiet
- `sudo pacman -Rs pakiet` : usuń pakiet i jego zależności
- `pacman -F "nazwa_pliku"` : poszukaj w bazie danych pakietów zawierających podany plik
- `pacman -Q` : wyświetl zainstalowane pakiety i ich wersje
- `pacman -Qe` : wyświetl tylko pakiety niebędące zależnościami i ich wersje
- `pacman -Qtdq` : wyświetl pakiety-sieroty (zainstalowane jako zależności ale nie są już wymagane przez żaden pakiet)
- `sudo pacman -Scc` : wyczyść całą pamięć podręczną pacmana

## Links
- More information: https://wiki.archlinux.org/title/Pacman/Rosetta
