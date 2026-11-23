# apt-mark

Narzędzie do zmiany statusu zainstalowanych pakietów.

## Usage
- `sudo apt-mark auto pakiet` : oznacz pakiet jako zainstalowany automatycznie
- `sudo apt-mark hold pakiet` : zatrzymaj pakiet w bieżącej wersji i zapobiegaj jego aktualizacjom
- `sudo apt-mark unhold pakiet` : zezwól, aby pakiet znowu był aktualizowany
- `apt-mark showmanual` : pokaż pakiety zainstalowane ręcznie
- `apt-mark showhold` : pokaż zatrzymane pakiety, które nie są aktualizowane

## Links
- More information: https://manned.org/apt-mark
