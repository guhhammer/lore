# at

Jednokrotnie wykonaj polecenia w późniejszym czasie. Usługa atd (lub atrun) powinna być uruchomiona dla rzeczywistych wykonań.

## Usage
- `at now + 5 minutes` : wykonaj polecenia z `stdin` po upływie 5 minut (naciśnij `<Ctrl d>` po zakończeniu)
- `echo "./zrób_kopię_zapasową_bazy_danych.sh" | at 1000` : wykonaj polecenie z `stdin` dziś o 10:00
- `at -f ścieżka/do/pliku 9:30 PM Tue` : wykonaj polecenia z danego pliku w następny wtorek

## Links
- More information: https://manned.org/at
