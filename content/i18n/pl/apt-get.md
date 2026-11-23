# apt-get

Narzędzie do zarządzania pakietami Debiana i Ubuntu. Szukaj pakietów używając `apt-cache`.

## Usage
- `sudo apt-get update` : zaktualizuj listę dostępnych pakietów oraz wersji (zalecane jest uruchomienie tego polecenia przed innymi poleceniami `apt-get`)
- `sudo apt-get install pakiet` : zainstaluj pakiet lub zaktualizuj go do najnowszej dostępnej wersji
- `sudo apt-get remove pakiet` : usuń pakiet
- `sudo apt-get purge pakiet` : usuń pakiet i jego pliki konfiguracyjne
- `sudo apt-get upgrade` : zaktualizuj wszystkie zainstalowane pakiety do ich najnowszych dostępnych wersji
- `sudo apt-get autoclean` : wyczyść lokalne repozytorium - usuwa wszystkie pliki pakietów (`.deb`) z przerwanych pobrań które nie mogą już być pobrane
- `sudo apt-get autoremove` : usuń wszystkie pakiety, które już nie są potrzebne
- `sudo apt-get dist-upgrade` : zaktualizuj zainstalowane pakiety (jak `upgrade`), ale usuń przestarzałe pakiety i zainstaluj dodatkowe pakiety, aby spełnić zależności

## Links
- More information: https://manned.org/apt-get.8
