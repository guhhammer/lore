# df

Wyświetl przegląd wykorzystania przestrzeni dyskowej systemu plików.

## Usage
- `df` : wyświetl wszystkie systemy plików i ich wykorzystanie dysków w jednostkach 512-bajtowych
- `df -h -c` : użyj jednostek czytelnych dla człowieka (z ang. human) (opartych na potęgach 1024) i wyświetl sumę całkowitą
- `df -H` : użyj jednostek czytelnych dla człowieka (z ang. Human) (opartych na potęgach 1000)
- `df ścieżka/do/pliku_lub_katalogu` : wyświetl wszystkie systemy plików i ich wykorzystanie dysków zawierające podany plik lub katalog
- `df -iT` : dołącz statystyki dotyczące liczby wolnych i wykorzystanych i-węzłów wraz z Typami systemów plików
- `df -k` : użyj jednostek 1024-bajtowych do wyświetlania danych o przestrzeni dyskowej
- `df -P` : wyświetl informację w sposób Przenośny

## Links
- More information: https://man.freebsd.org/cgi/man.cgi?query=df
