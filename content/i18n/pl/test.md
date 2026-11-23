# test

Sprawdza typy plików i porównuje wartości. Zwraca 0 gdy porównanie zwróciło wartość poprawną, 1 gdy fałszywą.

## Usage
- `test "$ZMIENNA" = "/bin/zsh"` : sprawdź czy podana zmienna jest równa łańcuchowi znaków
- `test -z "$GIT_BRANCH"` : sprawdź czy zmienna jest pusta
- `test -f "ścieżka/do/pliku"` : sprawdź czy plik istnieje
- `test ! -d "ścieżka/do/katalogu"` : sprawdź czy katalog nie istnieje
- `test warunek && echo "gdy poprawne" || echo "gdy fałszywe"` : zapis jeśli porawne-jeśli fałszywe

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/test-invocation.html
