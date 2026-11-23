# apk

Narzędzie zarządzania pakietami Alpine Linux.

## Usage
- `apk upgrade -U` : zaktualizuj indeksy repozytoriów i wszystkie pakiety
- `apk update` : zaktualizuj tylko indeksy repozytoriów
- `apk add pakiet` : zainstaluj nowy pakiet
- `apk del pakiet` : usuń pakiet
- `apk fix pakiet` : napraw/Zainstaluj ponownie pakiet bez modyfikacji głównych zależności
- `apk search -v słowo_kluczowe` : wyszukaj pakiety ze słowem kluczowym w ich nazwie i wyświetl wyniki z opisami
- `apk search -d słowo_kluczowe` : wyszukaj pakiety ze słowem kluczowym w ich opisie
- `apk info pakiet` : wyświetl informacje o określonym pakiecie

## Links
- More information: https://wiki.alpinelinux.org/wiki/Alpine_Package_Keeper
