# pacman-database

Operuj na bazie danych pakietów Arch Linuksa. Modyfikuj niektóre atrybuty zainstalowanych pakietów.

## Usage
- `sudo pacman -D --asdeps nazwa_pakietu` : oznacz pakiet jako pośrednio zainstalowany (zależność)
- `sudo pacman -D --asexplicit nazwa_pakietu` : oznacz pakiet jako bezpośrednio zainstalowany
- `pacman -Dk` : sprawdź, czy wszystkie zależności pakietów są zainstalowane
- `pacman -Dkk` : sprawdź repozytoria, aby zapewnić, że wszystkie podane zależności są dostępne
- `pacman -Dkq` : wyświetlaj tylko komunikaty o błędach
- `pacman -Dh` : wyświetl pomoc

## Links
- More information: https://manned.org/pacman.8
