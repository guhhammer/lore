# abbr

Zarządzaj skrótami dla powłoki fish. Zdefiniowane przez użytkownika słowa są zastępowane po wpisaniu dłuższymi zwrotami.

## Usage
- `abbr -a nazwa_skrótu polecenie argumenty_polecenia` : dodaj nowy skrót
- `abbr --rename stara_nazwa nowa_nazwa` : zmień nazwę istniejącego skrótu
- `abbr -e nazwa_skrótu` : usuń istniejący skrót
- `ssh nazwa_hosta abbr -s | source` : zaimportuj skróty zdefiniowane na innym hoście poprzez SSH

## Links
- More information: https://fishshell.com/docs/current/cmds/abbr.html
