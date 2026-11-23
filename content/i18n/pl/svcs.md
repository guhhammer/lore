# svcs

Wyświetl informację o uruchomionych usługach.

## Usage
- `svcs` : wypisz wszystkie uruchomione usługi
- `svcs -vx` : wypisz wszystkie usługi, które nie są uruchomione
- `svcs apache` : wypisz informacje o usłudze
- `svcs -L apache` : pokaż lokalizację pliku dziennika usługi
- `tail $(svcs -L apache)` : wyświetl koniec pliku dziennika usługi

## Links
- More information: https://www.unix.com/man-page/sunos/1/svcs
