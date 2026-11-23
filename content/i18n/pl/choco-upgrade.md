# choco upgrade

Zaktualizuj jeden lub więcej pakietów Chocolatey.

## Usage
- `choco upgrade pakiet1 pakiet2 ...` : zaktualizuj jeden lub więcej pakietów (oddzielonych spacją)
- `choco upgrade pakiet --version wersja` : zaktualizuj pakiet do konkretnej wersji
- `choco upgrade all` : zaktualizuj wszystkie pakiety
- `choco upgrade all --except "pakiet1 pakiet2 ..."` : zaktualizuj wszystkie pakiety z wyjątkiem tych podanych, rozdzielanych przecinkami
- `choco upgrade pakiet -y` : automatycznie akceptuj wszystkie monity podczas aktualizacji pakietu
- `choco upgrade pakiet -s adres_url|alias` : ustaw określone źródło/repozytorium pakietów
- `choco upgrade pakiet -u nazwa_użytkownika -p hasło` : podaj nazwę użytkownika i hasło do uwierzytelnienia

## Links
- More information: https://docs.chocolatey.org/en-us/choco/commands/upgrade/
