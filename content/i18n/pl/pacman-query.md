# pacman-query

Narzędzie do zarządzania pakietami w Arch Linuksie.

## Usage
- `pacman -Q` : wyświetl zainstalowane pakiety i ich wersje
- `pacman -Qe` : wyświetl tylko pakiety niebędące zależnościami i ich wersje
- `pacman -Qo nazwa_pliku` : znajdź, do którego pakietu należy plik
- `pacman -Qi nazwa_pakietu` : wyświetl informacje o zainstalowanym pakiecie
- `pacman -Ql nazwa_pakietu` : znajdź pliki należące do pakietu
- `pacman -Qdtq` : wyświetl pakiety-sieroty (zainstalowane jako zależności ale nie są już wymagane przez żaden pakiet)
- `pacman -Qm` : wyświetl zainstalowane pakiety, których nie ma w repozytoriach
- `pacman -Qu` : wyświetl przestarzałe pakiety

## Links
- More information: https://manned.org/pacman.8
