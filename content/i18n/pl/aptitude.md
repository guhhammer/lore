# aptitude

Narzędzie zarządzania pakietami dla Debiana i Ubuntu.

## Usage
- `sudo aptitude update` : zaktualizuj listę dostępnych pakietów oraz wersji. Zalecane jest uruchomienie tego polecenia przed innymi poleceniami `aptitude`
- `sudo aptitude install pakiet` : zainstaluj nowy pakiet i jego zależności
- `aptitude search pakiet` : wyszukaj pakiet
- `aptitude search '?installed(pakiet)'` : wyszukaj zainstalowany pakiet (`?installed` jest terminem wyszukiwania w `aptitude`)
- `sudo aptitude remove pakiet` : usuń pakiet i wszystkie pakiety zależne od niego
- `sudo aptitude upgrade` : zaktualizuj zainstalowane pakiety do najnowszej dostępnej wersji
- `sudo aptitude full-upgrade` : zaktualizuj zainstalowane pakiety (jak robi `aptitude upgrade`) włącznie z usunięciem przestarzałych pakietów i instalacją dodatkowych pakietów w celu spełnienia zależności
- `sudo aptitude hold '?installed(pakiet)'` : ustaw zainstalowany pakiet jako wstrzymany, aby zapobiec jego automatycznym aktualizacjom

## Links
- More information: https://manned.org/aptitude
