# diff

Porównaj pliki i foldery.

## Usage
- `diff stary_plik nowy_plik` : porównaj pliki (lista zmian między `stary_plik` a `nowy_plik`)
- `diff -w stary_plik nowy_plik` : porównaj pliki, ignoruj białe znaki (white spaces)
- `diff -y stary_plik nowy_plik` : porównaj pliki, pokaż różnice obok siebie
- `diff -u stary_plik nowy_plik` : porównaj pliki, pokaż różnice w ujednoliconym formacie (tak jak w przypadku `git diff`)
- `diff -r stary_folder nowy_folder` : porównaj foldery rekurencyjnie (pokazuje nazwy różniących się plików/folderów a także zmiany w plikach)
- `diff -r -q stary_folder nowy_folder` : porównaj foldery rekurencyjnie, pokaż tylko nazwy plików które się różnią

## Links
- More information: https://manned.org/diff
