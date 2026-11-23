# apt

Narzędzie do zarządzania pakietami dla dystrybucji bazujących na Debianie. Zalecany zamiennik `apt-get` przy użyciu interaktywnym w Ubuntu w wersjach 16.04 i wyższych.

## Usage
- `sudo apt update` : zaktualizuj listę dostępnych pakietów i ich wersji (zaleca się uruchomienie tego przed innymi poleceniami `apt`)
- `apt search pakiet` : wyszukaj podany pakiet
- `apt show pakiet` : wyświetl informacje o podanym pakiecie
- `sudo apt install pakiet` : zainstaluj pakiet lub zaktualizuj go do najnowszej dostępnej wersji
- `sudo apt remove pakiet` : usuń pakiet (użyj `purge` aby usunąć także pliki konfiguracyjne)
- `sudo apt upgrade` : zaktualizuj wszystkie zainstalowane pakiety do ich najnowszych wersji
- `apt list` : wyświetl wszystkie pakiety
- `apt list -i` : wyświetl zainstalowane pakiety

## Links
- More information: https://wiki.archlinux.org/title/Pacman/Rosetta
