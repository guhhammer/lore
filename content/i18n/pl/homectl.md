# homectl

Twórz, usuwaj, zmieniaj lub sprawdzaj katalogi domowe używając usługi systemd-homed.

## Usage
- `homectl list` : wyświetl konta użytkowników i ich powiązane katalogi domowe
- `sudo homectl create nazwa_użytkownika` : utwórz konto użytkownika i jego powiązany katalog domowy
- `sudo homectl remove nazwa_użytkownika` : usuń podanego użytkownika i jego powiązany katalog domowy
- `sudo homectl passwd nazwa_użytkownika` : zmień hasło podanego użytkownika
- `sudo homectl with nazwa_użytkownika -- komenda argumenty_dla_komendy` : uruchom powłokę lub komendę z dostępem do podanego katalogu domowego
- `sudo homectl lock|unlock nazwa_użytkownika` : zablokuj lub odblokuj podany katalog domowy
- `sudo homectl resize nazwa_użytkownika 100G` : zmień miejsce na dysku przydzielone dla podanego katalogu domowego na 100 GiB
- `homectl -h` : wyświetl pomoc

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/homectl.html
