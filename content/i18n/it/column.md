# column

Formatta `stdin` o un file in più colonne. Le colonne sono riempite prima delle righe; il separatore predefinito è lo spazio.

## Usage
- `printf "intestazione1 intestazione2\nbar foo\n" | column -c 30` : formatta l'output per uno schermo largo 30 caratteri
- `printf "intestazione1 intestazione2\nbar foo\n" | column -t` : separa colonne ed allinea automaticamente in un formato tabulare
- `printf "intestazione1 intestazione2\nbar foo\n" | column -t -s ,` : specifica un diverso separatore di colonna (e.g. "," per CSV) (il predefinito è lo spazio)
- `printf "intestazione1\nbar\nfoobar\n" | column -c 30 -x` : riempi le righe prima delle colonne

## Links
- More information: https://manned.org/column
