# cp

Kopiuje pliki i katalogi.

## Usage
- `cp ścieżka/do/pliku_źródłowego.ext ścieżka/do/pliku_docelowego.ext` : kopiuj plik do innej lokalizacji
- `cp ścieżka/do/pliku_źródłowego ścieżka/do/katalogu` : kopiuj plik do innego katalogu, zachowując nazwę pliku
- `cp -R ścieżka/do/katalogu_źródłowego ścieżka/do/katalogu_docelowego` : kopiuj rekursywnie katalog z zawartością do innej lokalizacji (jeśli docelowa lokalizacja istnieje, katalog jest kopiowany do jej środka)
- `cp -vR ścieżka/do/katalogu_źródłowego ścieżka/do/katalogu_docelowego` : kopiuj katalog rekursywnie, w trybie opisowym (pokazuje pliki które zostały skopiowane)
- `cp -i *.txt ścieżka/do/katalogu_docelowego` : kopiuj pliki tekstowe do innej lokalizacji, w trybie interaktywnym (wyświetla pytanie o potwierdzenie przed nadpisywaniem)
- `cp -L dowiązanie ścieżka/do/katalogu_docelowego` : podążaj za dowiązaniami symbolicznymi przed kopiowaniem

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/cp-invocation.html
