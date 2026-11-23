# choco source

Zarządzaj źródłami/repozytoriami pakietów Chocolatey.

## Usage
- `choco source list` : wyświetl aktualnie dostępne źródła
- `choco source add -n nazwa -s adres_url` : dodaj nowe źródło
- `choco source add -n nazwa -s adres_url -u nazwa_użytkownika -p hasło` : dodaj nowe źródło z użyciem poświadczeń
- `choco source add -n nazwa -s adres_url --cert ścieżka/do/certyfikatu` : dodaj nowe źródło z użyciem certyfikatu
- `choco source enable -n nazwa` : włącz dane źródło/repozytorium pakietów
- `choco source disable -n nazwa` : wyłącz dane źródło/repozytorium pakietów
- `choco source remove -n nazwa` : usuń dane źródło/repozytorium

## Links
- More information: https://docs.chocolatey.org/en-us/choco/commands/source/
