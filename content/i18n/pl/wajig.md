# wajig

Uproszczone narzędzie do zarządzania pakietami dla systemów oparych na Debianie.

## Usage
- `wajig update` : aktualizacja listy dostępnych pakietów i ich wersji
- `wajig install pakiet` : instalacja pakietu lub aktualizacja do najnowszej wersji
- `wajig purge pakiet` : usunięcie pakietu i jego plików konfiguracyjnych
- `wajig daily-upgrade` : wykonanie update, a następnie dist-upgrade
- `wajig sizes` : wyświetlenie rozmiaru zainstalowanych pakietów
- `wajig versions` : lista wersji i dystrybucji dla wszystkich zainstalowanych pakietów
- `wajig toupgrade` : lista wersji pakietów możliwych do aktualizacji
- `wajig dependents pakiet` : wyświetlenie pakietów, które posiadają zaleność od podanego pakietu

## Links
- More information: https://togaware.com/linux/survivor/wajig.html
