# ag

The Silver Searcher. Podobny do `ack`, ale ma być szybszy.

## Usage
- `ag foo` : znajdź pliki zawierające "foo" i wypisz dopasowane linie
- `ag foo ścieżka/do/katalogu` : znajdź pliki zawierające "foo" w określonym katalogu
- `ag -l foo` : znajdź pliki zawierające "foo", ale wypisz tylko nazwy plików
- `ag -i -o FOO` : znajdź pliki zawierające "FOO" bez rozróżniania wielkości liter i wypisz tylko dopasowanie, a nie całą linię
- `ag foo -G bar` : znajdź "foo" w plikach o nazwie pasującej do "bar"
- `ag '^ba(r|z)$'` : znajdź pliki, których zawartość pasuje do wyrażenia regularnego
- `ag -g foo` : znajdź pliki o nazwie pasującej do "foo"

## Links
- More information: https://manned.org/ag
