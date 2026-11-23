# prstat

Raportuj statystyki aktywnego procesu.

## Usage
- `prstat` : sprawdź wszystkie procesy i raportuj statystyki posortowane według użycia procesora
- `prstat -s rss` : sprawdź wszystkie procesy i raportuj statystyki posortowane według użycia pamięci
- `prstat -t` : raportuj podsumowanie całkowitego użycia dla każdego użytkownika
- `prstat -m` : raportuj informacje o pomiarach procesu mikrostanu
- `prstat -c -n 5 -s cpu 1` : wypisz 5 najbardziej obciążających procesor procesów co sekundę

## Links
- More information: https://www.unix.com/man-page/sunos/1m/prstat
