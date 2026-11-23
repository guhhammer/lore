# autoflake

Usuń nieużywane importy i zmienne z kodu Python.

## Usage
- `autoflake --remove-unused-variables ścieżka/do/pliku.py` : usuń nieużywane zmienne z jednego pliku i wyświetl różnicę
- `autoflake --remove-all-unused-imports ścieżka/do/pliku1.py ścieżka/do/pliku2.py ...` : usuń nieużywane importy z wielu plików i wyświetl różnice
- `autoflake --remove-unused-variables --in-place ścieżka/do/pliku.py` : usuń nieużywane zmienne z pliku, nadpisując plik
- `autoflake --remove-unused-variables --in-place --recursive ścieżka/do/katalogu` : usuń nieużywane zmienne rekurencyjnie ze wszystkich plików w katalogu, nadpisując każdy plik

## Links
- More information: https://github.com/PyCQA/autoflake#advanced-usage
