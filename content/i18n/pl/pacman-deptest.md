# pacman-deptest

Sprawdź każdą podaną zależność i zwróć listę zależności, które nie są aktualnie spełnione.

## Usage
- `pacman -T nazwa_pakietu1 nazwa_pakietu2 ...` : wyświetl nazwy zależności, które nie są zainstalowane
- `pacman -T "bash>=5"` : sprawdź, czy zainstalowany pakiet spełnia podaną minimalną wersję
- `pacman -T "bash>5"` : sprawdź, czy jest zainstalowana nowsza wersja pakietu
- `pacman -Th` : wyświetl pomoc

## Links
- More information: https://manned.org/pacman.8
