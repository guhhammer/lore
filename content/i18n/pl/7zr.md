# 7zr

Archiwizator plików o wysokim współczynniku kompresji. Podobny do `7z` z wyjątkiem tego, że obsługuje tylko pliki `7z`.

## Usage
- `7zr a ścieżka/do/archiwum.7z ścieżka/do/pliku_lub_katalogu` : zarchiwizuj plik lub katalog
- `7zr a ścieżka/do/zaszyfrowanego.7z -phasło -mhe=on ścieżka/do/archiwum.7z` : zaszyfruj istniejące archiwum (w tym nazwy plików)
- `7zr x ścieżka/do/archiwum.7z` : wypakuj archiwum, zachowując oryginalną strukturę katalogów
- `7zr x ścieżka/do/archiwum.7z -ościeżka/do/wyjścia` : wypakuj archiwum do określonego katalogu
- `7zr x ścieżka/do/archiwum.7z -so` : wypakuj archiwum do `stdout`
- `7zr l ścieżka/do/archiwum.7z` : wypisz zawartość pliku archiwum
- `7zr a ścieżka/do/archiwum.7z -mx=0|1|3|5|7|9 ścieżka/do/pliku_lub_katalogu` : ustaw poziom kompresji (wyższy oznacza wyższą kompresję, ale wolniejszą)

## Links
- More information: https://manned.org/7zr
