# exa

Nowoczesny odpowiednik `ls` (wyświetla zawartość katalogu).

## Usage
- `exa -1` : wyświetl listę plików, po jednym w linii
- `exa -a` : wyświetl wszystkie pliki, łącznie z ukrytymi
- `exa -l -a` : wyświetl listę wszystkich plików ze szczegółami (uprawnienia, właściciel, wielkość i data zmiany)
- `exa -r -s size` : wyświetl listę plików posortowaną względem wielkości pliku, od największego
- `exa -l -T -L 3` : wyświetl drzewko plików (trzy poziomy)
- `exa -l -s modified` : wyświetl listę plików posortowaną względem daty zmiany, od najstarszego
- `exa -l -h --icons --git` : wyświetl listę plików wraz z nagłówkiem, ikoną i statusem Git
- `exa --git-ignore` : wyświetl listę plików, ignorując pliki z `.gitignore`

## Links
- More information: https://github.com/ogham/exa#command-line-options
