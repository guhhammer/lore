# choco install

Instalacja jednej lub więcej paczek zarządzanych przez Chocolatey.

## Usage
- `choco install nazwa_pakietu(pakietów)` : instalacja jednego lub więcej pakietów (oddzielonych spacją)
- `choco install ścieżka/do/plik_konfiguracyjny.config` : instakacja pakietów z użyciem podanego pliku konfiguracyjnego
- `choco install ścieżka/do/pliku` : instalacja podanej specyfikacji `.nuspec` lub pliku `.nupkg`
- `choco install pakiet --version wersja` : instalacja konkretnej podanej wersji pakietu
- `choco install pakiet --allow-multiple` : zezwól na instalacjie wielu wersji danego pakietu
- `choco install pakiet -y` : automatyczna akceptacja wszystkich monitów podczas instalacji
- `choco install pakiet -s adres_url|alias` : ustawienie określonego źródła/repozytorium pakietów
- `choco install pakiet -u nazwa_użytkownika -p hasło` : podanie nazwy użytkownika i hasła do uwierzytelnienia

## Links
- More information: https://docs.chocolatey.org/en-us/choco/commands/install/
