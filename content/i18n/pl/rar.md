# rar

Archiwizator RAR. Obsługuje wielotomowe archiwa, które mogą być opcjonalnie samorozpakowujące się.

## Usage
- `rar a ścieżka/do/nazwa_archiwum.rar ścieżka/do/pliku1 ścieżka/do/pliku2 ścieżka/do/pliku3 ...` : zarchiwizuj 1 lub więcej plików
- `rar a ścieżka/do/nazwa_archiwum.rar ścieżka/do/katalogu` : zarchiwizuj katalog
- `rar a -v50M -R ścieżka/do/nazwa_archiwum.rar ścieżka/do/pliku_lub_katalogu` : podziel archiwum na części równej wielkości (50M)
- `rar a -phasło ścieżka/do/nazwa_archiwum.rar ścieżka/do/pliku_lub_katalogu` : chroń hasłem powstające archiwum
- `rar a -hphasło ścieżka/do/nazwa_archiwum.rar ścieżka/do/pliku_lub_katalogu` : szyfruj dane pliku i ich nagłówki za pomocą hasła
- `rar a -m0..5 ścieżka/do/nazwa_archiwum.rar ścieżka/do/pliku_lub_katalogu` : użyj określonego poziomu kompresji (0-5)

## Links
- More information: https://manned.org/rar
